use std::rc::Rc;

use crate::error;
use crate::error::Error;
use crate::itertools::accumulate::accumulate;
use crate::sequence::Sequence;
use crate::utils::extract_value_from_result_vec;


#[allow(dead_code)]
pub(crate) struct DivideInner<T> {
    buf: Box<dyn Sequence<T>>,
    pub(crate) n: usize,
    len_vec: Vec<usize>,
    accumulate_overflow: bool
}

pub struct Divide<T> {
    pub(crate) inner: Rc<DivideInner<T>>
}

impl<T> Divide<T> 
where
T: Clone + 'static
{
    pub fn new(buf: Box<dyn Sequence<T>>, bucket_count: usize) -> Divide<T> {
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


        let inner = DivideInner {
            buf: buf,
            n: bucket_count,
            len_vec: _len_vec2,
            accumulate_overflow: accumulate_overflow
        };

        let ret = Divide {
            inner: Rc::new(inner)
        };

        return ret;
    }

    pub fn iter_cnt(&self) -> usize {
        return self.inner.n;
    }

    pub fn iter(&self, bucket_no: usize) -> Box<dyn Iterator<Item = Result<T, Error>>> {
        assert!(bucket_no < self.inner.len_vec.len() - 1);
        let start = self.inner.len_vec[bucket_no];
        let end = self.inner.len_vec[bucket_no+1];

        let ret: Box<dyn Iterator<Item = Result<T, Error>>> = Box::new(Cursor {
            inner: Rc::clone(&self.inner),
            cur: start,
            end: end,
            bucket_count: self.inner.n,
            accumulate_overflow: self.inner.accumulate_overflow
        });

        return ret;
    }
}

pub fn divide<T>(buf: Box<dyn Sequence<T>>, bucket_cnt: usize) -> Divide<T>
where
T: Clone + 'static
{
    return Divide::new(buf, bucket_cnt);
}

pub struct Cursor<T>
{
    inner: Rc<DivideInner<T>>,
    cur: usize,
    end: usize,
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

        let real_ret: Option<Result<_, _>> = Some(Ok(self.inner.buf.get(self.cur).unwrap().clone()));

        self.cur += 1;

        return real_ret;
    }
}

impl<T> Drop for Cursor<T> {
    fn drop(&mut self) {
        println!("Cursor, refcnt={}", Rc::strong_count(&self.inner));
    }
}
impl<T> Drop for Divide<T> {
    fn drop(&mut self) {
        println!("Divide, refcnt={}", Rc::strong_count(&self.inner));
    }
}


#[cfg(test)]
mod tests {
    use crate::sequence::create_seq_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let seq = create_seq_from_vec(v);
        let div = divide(seq, 3);

        let mut cur_0 = div.iter(0);
        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(2, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(3, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(4, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());


        let mut cur_1 = div.iter(1);
        assert_eq!(5, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(6, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(7, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());

        let mut cur_2 = div.iter(2);
        assert_eq!(8, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(9, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(10, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_2.next());
    }

    #[test]
    fn test2() {
        let v = create_seq_from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
        let div = divide(v, 3);

        let mut cur_0 = div.iter(0);
        println!("{:?}", cur_0.next());
    }
}
