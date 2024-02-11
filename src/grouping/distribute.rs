use std::isize;

use crate::error;
use crate::error::Error;

pub struct Cursor<T>
{
    cur: *const T,
    end: *const T,
    step: usize
}

impl<T> Cursor<T>
where T: Clone
{
    pub fn next(&mut self) -> Result<Option<T>, Error> {
        if self.cur >= self.end {
            return Ok(None);
        }

        if ((self.step * std::mem::size_of::<T>()) as isize) < 0 {
            return Err(error::overflow_error("Overflow when check self.step.".to_string()));
        }

        let ret = (self.cur as usize).overflowing_add(self.step * std::mem::size_of::<T>());
        if ret.1 {
            // overflow
            return Err(error::overflow_error("Overflow when move the cursor.".to_string()));
        } else {
            // no overflow
            let pointer = self.cur;
            unsafe {
                self.cur = self.cur.offset(self.step as isize);
                return Ok(Some((*pointer).clone()));
            }
        }
    }
}

pub struct Distribute<T> {
    buf: Vec<T>,
    cursor_table: Vec<Cursor<T>>
}

impl<T> Distribute<T> {
    pub  fn new(buf: Vec<T>, cursor_cnt: usize) -> Self {
        let mut ret = Distribute{
            buf: buf,
            cursor_table: Vec::new()
        };

        unsafe {
            let start: *const T = ret.buf.as_ptr();
            let end: *const T = start.offset(ret.buf.len() as isize);

            for i in 0..cursor_cnt {
                let cursor = Cursor {
                    cur: start.offset(i as isize),
                    end: end,
                    step: cursor_cnt
                };
                ret.cursor_table.push(cursor);
            }
        }
        
        return ret;
    }

    pub fn get_cursor_cnt(&self) -> usize {
        return self.cursor_table.len();
    }

    pub fn get_cursor(&self, i: usize) -> &Cursor<T> {
        assert!(i < self.cursor_table.len());

        return &self.cursor_table[i];
    }

    pub fn get_cursor_mut(&mut self, i: usize) -> &mut Cursor<T> {
        assert!(i < self.cursor_table.len());

        return &mut self.cursor_table[i];
    }
}

pub fn distribute<T>(buf: Vec<T>, bucket_cnt: usize) -> Distribute<T> 
{
    return Distribute::new(buf, bucket_cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let mut dist: Distribute<i32> = distribute(v, 3);

        for i in 0..3 {
            let cur_0: &mut Cursor<i32> = dist.get_cursor_mut(i);

            loop {
                let ret = cur_0.next();
                match ret {
                    Err(e) => { println!("{:?}", e); },
                    Ok(v) => {
                        match v {
                            None => { break; }
                            Some(v2) => {
                                println!("{:?}", v2);
                            }
                        }
                    }
                }
            }
        }   
    }

}