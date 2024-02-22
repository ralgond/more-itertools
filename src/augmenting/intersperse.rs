use std::collections::VecDeque;
use std::fmt::Debug;

use crate::error;
use crate::error::Error;

#[derive(Debug)]
struct IntersperseOutputBuffer<T> 
where
T: Debug
{
    items: VecDeque<T>
}

#[derive(Debug)]
pub struct Intersperse<I: Iterator> 
where
I::Item: Clone + Debug
{
    buffer: IntersperseOutputBuffer<I::Item>,
    iter: I,
    n: usize,
    e: I::Item,
    iter_finished: bool,
    emit_count: usize
}

impl<I: Iterator> Iterator for Intersperse<I>
where
I::Item: Clone + Debug
{
    type Item = Result<<I as Iterator>::Item, Error>;

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

pub fn intersperse<I>(e: I::Item, iterable: I, n: usize) -> Intersperse<I::IntoIter>
where
I: IntoIterator,
I::Item: Clone + Debug
{
    let isp = Intersperse {
        buffer: IntersperseOutputBuffer {
            items: VecDeque::new(),
        },
        iter: iterable.into_iter(),
        n: n,
        e: e,
        iter_finished: false,
        emit_count: 0
    };

    return isp;
}

#[cfg(test)]
mod tests {
    use crate::utils::extract_value_from_result_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut isp = intersperse(0, vec![1,2,3,4,5], 0);
        assert_eq!(error::Kind::ValueError, isp.next().unwrap().err().unwrap().kind());

        let mut isp = intersperse(0, Vec::<i32>::new(), 1);
        assert_eq!(None, isp.next());

        let isp = intersperse(0, vec![1,2,3,4,5], 1);
        assert_eq!((vec![1, 0, 2, 0, 3, 0, 4, 0, 5], false), extract_value_from_result_vec(isp.collect::<Vec<_>>()));

        let isp = intersperse(0, vec![1,2,3,4,5], 2);
        assert_eq!((vec![1, 2, 0, 3, 4, 0, 5], false), extract_value_from_result_vec(isp.collect::<Vec<_>>()));

        let isp = intersperse(0, vec![1], 1);
        assert_eq!((vec![1], false), extract_value_from_result_vec(isp.collect::<Vec<_>>()));

        let isp = intersperse(0, vec![1], 2);
        assert_eq!((vec![1], false), extract_value_from_result_vec(isp.collect::<Vec<_>>()));
    }
}