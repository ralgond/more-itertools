use std::collections::VecDeque;
use crate::error::Error;

use crate::selecting::take::take;


pub struct ZipOffset<T> {
    buf: VecDeque<T>,
    buf2: VecDeque<Option<T>>,
    iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>,
    iter_finished: bool,
    longest: bool,
    fillvalue: T
}

impl<T> Iterator for ZipOffset<T> 
where
T: Clone + 'static
{
    type Item = Result<Vec<T>,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.buf.len() > 0 {
                // let ret = self.buf.pop_front().unwrap();
                let mut ret = Vec::<T>::new();
                while self.buf.len() > 0 {
                    ret.push(self.buf.pop_front().unwrap());
                }
                return Some(Ok(ret));
            }

            assert_eq!(0, self.buf2.len());

            for i in self.iter_vec.iter_mut() {
                if let Some(v) = i.next() {
                    match v {
                        Ok(ok_v) => {
                            self.buf2.push_back(Some(ok_v));
                        },
                        Err(err_v) => {
                            self.iter_finished = true;
                            return Some(Err(err_v));
                        }
                    }
                } else {
                    self.buf2.push_back(None);
                }
            }

            if self.longest {
                if self.buf2.iter().all(|x| match x {
                    None => { return true; },
                    Some(_) => { return false; }
                }) {
                    self.iter_finished = true;
                } else {
                    while self.buf2.len() > 0 {
                        match self.buf2.pop_front() {
                            None => {
                                self.buf.push_back(self.fillvalue.clone());
                            },
                            Some(v) => {
                                if !v.is_none() {
                                    self.buf.push_back(v.unwrap());
                                } else { 
                                    self.buf.push_back(self.fillvalue.clone());
                                }
                            }
                        }
                    }
                }
            } else {
                if self.buf2.iter().any(|x| match x {
                    None => { return true; },
                    Some(_) => { return false; }
                }) {
                    self.iter_finished = true;
                } else {
                    while self.buf2.len() > 0 {
                        self.buf.push_back(self.buf2.pop_front().unwrap().unwrap());
                    }
                }
            }
        }   
    }
}

pub fn zip_offset<T>(mut iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>,
            offsets_vec: Vec<usize>,
            longest: bool,
            fillvalue: T) -> Box<dyn Iterator<Item = Result<Vec<T>,Error>>> 
where T: Clone + 'static            
{
    for (index, offset) in offsets_vec.iter().enumerate() {
        take(iter_vec.get_mut(index).as_mut().unwrap(), *offset);
    }

    return Box::new(ZipOffset {
        buf: VecDeque::new(),
        buf2: VecDeque::new(),
        iter_vec,
        iter_finished: false,
        longest,
        fillvalue
    });
}



#[cfg(test)]
mod tests {

    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(generate_okok_iterator(v1));
        v.push(generate_okok_iterator(v2));

        let ret = zip_offset(v, vec![0,1], false, '9');
        let ret2 = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert_eq!(vec![vec!['0', 'b'], vec!['1', 'c'], vec!['2', 'd'], vec!['3', 'e']], ret2.0);

        // =================================================

        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(generate_okok_iterator(v2));
        v.push(generate_okok_iterator(v1));

        let ret = zip_offset(v, vec![0,1], false, '9');
        let ret2 = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert_eq!(vec![vec!['a', '1'], vec!['b', '2'], vec!['c', '3']], ret2.0);
        assert_eq!(None, ret2.1);
    }

    #[test]
    fn test2() {
        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(generate_okok_iterator(v1));
        v.push(generate_okok_iterator(v2));

        let ret = zip_offset(v, vec![0,1], true, '9');
        let ret2 = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert_eq!(vec![vec!['0', 'b'], vec!['1', 'c'], vec!['2', 'd'], vec!['3', 'e'], vec!['9', 'f']], ret2.0);
    }

    #[test]
    fn test3() {
        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(generate_okokerr_iterator(v1, error::overflow_error("[test]".to_string())));
        v.push(generate_okok_iterator(v2));

        let ret = zip_offset(v, vec![0,1], false, '9');
        let ret2 = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert_eq!(vec![vec!['0', 'b'], vec!['1', 'c'], vec!['2', 'd'], vec!['3', 'e']], ret2.0);
        assert_eq!(error::Kind::OverflowError, ret2.1.unwrap().kind());


        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(generate_okok_iterator(v1));
        v.push(generate_okokerr_iterator(v2, error::overflow_error("[test]".to_string())));
        
        let ret = zip_offset(v, vec![0,1], false, '9');
        let ret2 = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert_eq!(vec![vec!['0', 'b'], vec!['1', 'c'], vec!['2', 'd'], vec!['3', 'e']], ret2.0);
        assert_eq!(None, ret2.1);
    }
}

