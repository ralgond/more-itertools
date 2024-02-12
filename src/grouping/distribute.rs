use crate::error;
use crate::error::Error;

pub struct Distribute<T> {
    buf: Vec<T>,
    bucket_count: usize,
    start: *const T,
    end: *const T
}

pub struct Cursor<T>
{
    cur: *const T,
    start: *const T,
    end: *const T,
    step: usize
}

impl<T> Cursor<T>
where
T: Clone
{
    pub fn offset(&self, count: isize) -> Self {
        let mut ret = Self { 
            cur: self.cur.clone(), 
            start: self.start.clone(), 
            end: self.end.clone(), 
            step: self.step.clone() };

        unsafe {
            ret.cur = ret.cur.offset(count);
        }
        
        return ret;
    }
}

impl<T> Iterator for Cursor<T>
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            return None;
        }

        if ((self.step * std::mem::size_of::<T>()) as isize) < 0 {
            return Some(Err(error::overflow_error("Overflow when check self.step.".to_string())));
        }

        let ret = (self.cur as usize).overflowing_add(self.step * std::mem::size_of::<T>());
        if ret.1 {
            // overflow
            return Some(Err(error::overflow_error("Overflow when move the cursor.".to_string())));
        } else {
            // no overflow
            let pointer = self.cur;
            unsafe {
                self.cur = self.cur.offset(self.step as isize);
                return Some(Ok((*pointer).clone()));
            }
        }
    }
}

impl<T> Distribute<T> 
where
T: Clone + 'static
{
    pub fn new(buf: Vec<T>, bucket_count: usize) -> Self {
        unsafe  {
            let start = buf.as_ptr();
            let end = start.offset(buf.len() as isize);

            let ret = Distribute {
                buf: buf,
                bucket_count: bucket_count,
                start: start,
                end: end
            };

            return ret;
        }
    }

    pub fn iter_cnt(&self) -> usize {
        return self.bucket_count;
    }

    pub fn iter(&self) -> Cursor<T> {
        let ret = Cursor {
            cur: self.start,
            start: self.start,
            end: self.end,
            step: self.bucket_count
        };

        return ret;
    }
}

pub fn distribute<T>(buf: Vec<T>, bucket_cnt: usize) -> Distribute<T> 
where
T: Clone + 'static
{
    return Distribute::new(buf, bucket_cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let dist: Distribute<i32> = distribute(v, 3);

        let mut cur_0 = dist.iter();
        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(4, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(7, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(10, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let dist: Distribute<i32> = distribute(v, 3);

        let mut cur_0 = dist.iter();
        let mut cur_1 = cur_0.offset(1);
        let mut cur_2 = cur_1.offset(1);
        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(2, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(3, cur_2.next().unwrap().ok().unwrap());


        assert_eq!(4, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(5, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(6, cur_2.next().unwrap().ok().unwrap());
        
        
        assert_eq!(7, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(8, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(9, cur_2.next().unwrap().ok().unwrap());

        assert_eq!(10, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());
        
    }
}