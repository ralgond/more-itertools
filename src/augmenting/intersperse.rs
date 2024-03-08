use std::collections::VecDeque;

use crate::error;
use crate::error::Error;

struct IntersperseOutputBuffer<T> 
{
    items: VecDeque<T>
}

pub struct Intersperse<T> 
where
T: Clone + 'static
{
    buffer: IntersperseOutputBuffer<T>,
    iter: Box<dyn Iterator<Item=T>>,
    n: usize,
    e: T,
    iter_finished: bool,
    emit_count: usize
}

impl<T> Iterator for Intersperse<T>
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return Some(Err(error::value_error("n must be > 0".to_string())));
        }

        loop {
            if self.buffer.items.len() > 0 {
                if self.emit_count == self.n {
                    self.emit_count = 0;
                    return Some(Ok(self.e.clone()));
                }
                let ret = self.buffer.items.pop_front().unwrap();
                self.emit_count += 1;
                return Some(Ok(ret));
            } else {
                let _next = self.iter.next();
                match _next {
                    None => { 
                        self.iter_finished = true;
                        return None; 
                    }
                    Some(v) => {
                        self.buffer.items.push_back(v);
                    }
                }
            }
        }
    }
}

pub fn intersperse<T>(e: T, iter: Box<dyn Iterator<Item=T>>, n: usize) -> Box<dyn Iterator<Item=Result<T, Error>>>
where
T: Clone + 'static
{
    let isp = Intersperse {
        buffer: IntersperseOutputBuffer {
            items: VecDeque::new(),
        },
        iter,
        n,
        e,
        iter_finished: false,
        emit_count: 0
    };

    return Box::new(isp);
}

#[cfg(test)]
mod tests {
    use crate::{itertools::iter::iter_from_vec, utils::extract_value_from_result_vec};

    use super::*;

    #[test]
    fn test1() {
        let mut isp = intersperse(0, iter_from_vec(vec![1,2,3,4,5]), 0);
        assert_eq!(error::Kind::ValueError, isp.next().unwrap().err().unwrap().kind());

        let mut isp = intersperse(0, iter_from_vec(Vec::<i32>::new()), 1);
        assert_eq!(None, isp.next());

        let isp = intersperse(0, iter_from_vec(vec![1,2,3,4,5]), 1);
        assert_eq!((vec![1, 0, 2, 0, 3, 0, 4, 0, 5], None), extract_value_from_result_vec(isp.collect::<Vec<_>>()));

        let isp = intersperse(0, iter_from_vec(vec![1,2,3,4,5]), 2);
        assert_eq!((vec![1, 2, 0, 3, 4, 0, 5], None), extract_value_from_result_vec(isp.collect::<Vec<_>>()));

        let isp = intersperse(0, iter_from_vec(vec![1]), 1);
        assert_eq!((vec![1], None), extract_value_from_result_vec(isp.collect::<Vec<_>>()));

        let isp = intersperse(0, iter_from_vec(vec![1]), 2);
        assert_eq!((vec![1], None), extract_value_from_result_vec(isp.collect::<Vec<_>>()));
    }
}