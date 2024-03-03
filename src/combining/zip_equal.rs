use std::collections::VecDeque;
use crate::error;
use crate::error::Error;

pub struct ZipEqual<T> {
    buf: VecDeque<T>,
    buf2: VecDeque<Option<T>>,
    iter_vec: Vec<Box<dyn Iterator<Item = T>>>,
    iter_finished: bool
}

impl<T> Iterator for ZipEqual<T> 
where
T: Clone + 'static
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            // if self.buf.len() > 0 {
            //     let ret = self.buf.pop_front().unwrap();
            //     return Some(ret);
            // }

            assert_eq!(0, self.buf2.len());

            for i in self.iter_vec.iter_mut() {
                match i.next() {
                    None => {
                        self.buf2.push_back(None);
                    },
                    Some(v) => {
                        self.buf2.push_back(Some(v));
                    }
                }
            }

            if self.buf2.iter().all(|x| match x {
                None => { return true; },
                Some(_) => { return false; }
            }) {
                self.iter_finished = true;
            } else {
                while self.buf2.len() > 0 {
                    match self.buf2.front() {
                        None => {
                            self.buf2.pop_front();
                            continue;
                        },
                        Some(v) => {
                            match v {
                                None => {
                                    self.buf2.pop_front();
                                    continue;
                                },
                                Some(v2) => {
                                    self.buf.push_back(v2.clone());
                                    self.buf2.pop_front();
                                }
                            }
                        }
                    }
                }
                if self.buf.len() != self.iter_vec.len() {
                    self.iter_finished = true;
                    return Some(Err(error::any_error(error::Kind::IteratorError, "Iterables have different lengths".to_string())));
                } else {
                    let mut ret = Vec::new();
                    while self.buf.len() > 0 {
                        ret.push(self.buf.pop_front().unwrap());
                    }
                    return Some(Ok(ret));
                }
            }
        }   
    }
}

pub fn zip_equal<T>(iter_vec: Vec<Box<dyn Iterator<Item = T>>>) -> Box<dyn Iterator<Item = Result<Vec<T>,Error>>> 
where T: Clone + 'static
{
    Box::new(ZipEqual {
        buf: VecDeque::new(),
        buf2: VecDeque::new(),
        iter_vec,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut v = Vec::new();
        v.push(iter_from_vec(vec![1,2,3]));
        v.push(iter_from_vec(vec![4,5]));
        v.push(iter_from_vec(vec![6,7,8]));

        let mut iter = zip_equal(v);

        assert_eq!(Some(Ok(vec![1, 4, 6])), iter.next());
        assert_eq!(Some(Ok(vec![2, 5, 7])), iter.next());
        assert_eq!(error::Kind::IteratorError, iter.next().unwrap().err().unwrap().kind());
        assert_eq!(None, iter.next());
    }
}