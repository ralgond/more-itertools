use std::mem::swap;

use crate::error::Error;
use crate::error;

pub struct Chunked<T> {
    iter: Box<dyn Iterator<Item = T>>,
    buf: Vec<T>,
    n: usize,
    strict: bool
}


impl<T> Iterator for Chunked<T> {
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return Some(Err(error::value_error(String::from("n should not be 0"))));
        }

        let mut ret = Vec::new();
        while self.buf.len() < self.n {
            let ele = self.iter.next();
            match ele {
                Some(ele_value) => { self.buf.push(ele_value) }
                None => {
                    if self.buf.len() == 0 {
                        return None;
                    } else {
                        swap(&mut ret, &mut self.buf);
                        self.buf = Vec::new();
                        if self.strict && ret.len() < self.n {
                            return Some(Err(error::value_error(String::from("iterable is not divisible by n."))));
                        }
                        return Some(Ok(ret));
                    }
                }
            }

            if self.buf.len() == self.n {
                ret = Vec::new();
                swap(&mut ret, &mut self.buf);
                return Some(Ok(ret));
            }
        }

        return Some(Ok(ret));

    }
}

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#chunked
pub fn chunked<T>(iter: Box<dyn Iterator<Item = T>>, n: usize, strict: bool) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where
    T: 'static,
{
    Box::new(Chunked {
        iter,
        buf: Vec::new(),
        n,
        strict
    })
}


#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1_no_strict() {
        let mut it = chunked(iter_from_vec(vec![1,2,3,4,5,6,7,8,9,10]), 3, false);

        assert_eq!(vec![1,2,3], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![10], it.next().unwrap().ok().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test2_strict() {
        let mut it = chunked(iter_from_vec(vec![1,2,3,4,5,6,7,8,9,10]), 3, true);

        assert_eq!(vec![1,2,3], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], it.next().unwrap().ok().unwrap());

        assert_eq!(error::Kind::ValueError, it.next().unwrap().err().unwrap().kind());
    }

    #[test]
    fn test3_no_strict_chars() {
        let mut it = chunked(iter_from_vec("abcdefghij".chars().collect()), 3, false);

        assert_eq!(vec!['a', 'b', 'c'], it.next().unwrap().ok().unwrap());
        assert_eq!(vec!['d', 'e', 'f'], it.next().unwrap().ok().unwrap());
        assert_eq!(vec!['g', 'h', 'i'], it.next().unwrap().ok().unwrap());
        assert_eq!(vec!['j'], it.next().unwrap().ok().unwrap());

        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test3_no_strict_string() {
        let v = vec![String::from("1"),
                            String::from("2"),
                            String::from("3"),
                            String::from("4")
                            ];

        let mut it = chunked(iter_from_vec(v), 3, false);

        assert_eq!(vec![String::from("1"), String::from("2") ,String::from("3")], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![String::from("4")], it.next().unwrap().ok().unwrap());

        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test4_value_error_n_is_0() {
        let mut it = chunked(iter_from_vec(vec![1,2,3,4,5,6,7,8,9,10]), 0, false);
        assert_eq!(error::Kind::ValueError, it.next().unwrap().err().unwrap().kind());
    }
}

