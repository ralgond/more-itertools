use std::mem::swap;

use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Chunked<I: Iterator> {
    buf: Vec<I::Item>,
    n: usize,
    iter: I,
    strict: bool
}


impl<I: Iterator> Iterator for Chunked<I> {
    type Item = Result<Vec<<I as Iterator>::Item>, Error>;

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
pub fn chunked<I>(iterable: I, n: usize, strict: bool) -> Chunked<I::IntoIter>
where
    I: IntoIterator,
{
    Chunked {
        buf: Vec::new(),
        n: n,
        iter: iterable.into_iter(),
        strict: strict
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_no_strict() {
        let mut it = chunked(vec![1,2,3,4,5,6,7,8,9,10], 3, false);

        assert_eq!(vec![1,2,3], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![10], it.next().unwrap().ok().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test2_strict() {
        let mut it = chunked(vec![1,2,3,4,5,6,7,8,9,10], 3, true);

        assert_eq!(vec![1,2,3], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], it.next().unwrap().ok().unwrap());

        assert_eq!(error::Kind::ValueError, it.next().unwrap().err().unwrap().kind());
    }

    #[test]
    fn test3_no_strict_chars() {
        let mut it = chunked("abcdefghij".chars(), 3, false);

        assert_eq!(vec!['a', 'b', 'c'], it.next().unwrap().ok().unwrap());
        assert_eq!(vec!['d', 'e', 'f'], it.next().unwrap().ok().unwrap());
        assert_eq!(vec!['g', 'h', 'i'], it.next().unwrap().ok().unwrap());
        assert_eq!(vec!['j'], it.next().unwrap().ok().unwrap());

        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test3_no_strict_string() {
        let v: [String; 4] = [String::from("1"),
                            String::from("2"),
                            String::from("3"),
                            String::from("4")
                            ];

        let mut it = chunked(v, 3, false);

        assert_eq!(vec![String::from("1"), String::from("2") ,String::from("3")], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![String::from("4")], it.next().unwrap().ok().unwrap());

        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test4_value_error_n_is_0() {
        let mut it = chunked(vec![1,2,3,4,5,6,7,8,9,10], 0, false);
        assert_eq!(error::Kind::ValueError, it.next().unwrap().err().unwrap().kind());
    }
}

