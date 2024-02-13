use crate::error;
use crate::error::Error;
use crate::accumulate::accumulate;
use crate::utils::extract_value_from_result_vec;


pub struct Divide<T> {
    pub(crate) buf: Vec<T>,
    pub(crate) n: usize,
    start: *const T,
    end: *const T,
    len_vec: Vec<usize>,
    base: usize,
    accumulate_overflow: bool
}

impl<T> Divide<T> 
where
T: Clone + 'static
{
    pub fn new(buf: Vec<T>, bucket_count: usize) -> Divide<T> {
        let start;
        let end;
        unsafe  {
            start = buf.as_ptr();
            end = start.offset(buf.len() as isize);
        }

        let mut _len_vec = Vec::new();
        let base = buf.len() / bucket_count;
        let _mod = buf.len() % bucket_count;

        for _ in 0..bucket_count {
            _len_vec.push(base);
        }
        for i in 0.._mod {
            _len_vec[i] = _len_vec[i] + 1;
        }


        let mut accumulate_overflow = false; 
        let a = accumulate(_len_vec);
        let mut _len_vec3 = extract_value_from_result_vec(a.collect::<Vec<_>>());
        println!("{:?}", _len_vec3);
        if _len_vec3.1 {
            accumulate_overflow = true;
        }

        let mut _len_vec2 = Vec::new();
        _len_vec2.push(0 as usize);
        _len_vec2.append(&mut _len_vec3.0);


        let ret = Divide {
            buf: buf,
            n: bucket_count,
            start: start,
            end: end,
            len_vec: _len_vec2,
            base: base,
            accumulate_overflow: accumulate_overflow
        };

        return ret;
    }

    pub fn iter_cnt(&self) -> usize {
        return self.n;
    }

    pub fn iter(&self, bucket_no: usize) -> Cursor<T> {
        assert!(bucket_no < self.len_vec.len() - 1);
        let start = self.len_vec[bucket_no];
        let end = self.len_vec[bucket_no+1];
        unsafe {
            let ret = Cursor {
                cur: self.start.offset(start as isize),
                end: self.start.offset(end as isize),
                bucket_count: self.n,
                accumulate_overflow: self.accumulate_overflow
            };

            return ret;
        }
    }
}

pub fn divide<T>(buf: Vec<T>, bucket_cnt: usize) -> Divide<T>
where
T: Clone + 'static
{
    return Divide::new(buf, bucket_cnt);
}

pub struct Cursor<T>
{
    cur: *const T,
    end: *const T,
    bucket_count: usize,
    accumulate_overflow: bool
}

impl<T> Iterator for Cursor<T>
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bucket_count == 0 {
            return Some(Err(error::value_error("bucket count should not be 0".to_string())));
        }

        if self.accumulate_overflow {
            return Some(Err(error::value_error("accumulate overflow".to_string())));
        }

        if self.cur >= self.end {
            return None;
        }

        let pointer = self.cur;
        unsafe {
            self.cur = self.cur.offset(1 as isize);
            return Some(Ok((*pointer).clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let dist = divide(v, 3);

        let mut cur_0 = dist.iter(0);
        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(2, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(3, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(4, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());


        let mut cur_1 = dist.iter(1);
        assert_eq!(5, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(6, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(7, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());

        let mut cur_2 = dist.iter(2);
        assert_eq!(8, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(9, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(10, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_2.next());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let dist = divide(v, 3);

        let mut cur_0 = dist.iter(0);
        println!("{:?}", cur_0.next());
    }
}
