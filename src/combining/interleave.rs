use std::collections::VecDeque;

use crate::error::Error;


pub struct Interleave<T> {
    buf: VecDeque<T>,
    buf2: VecDeque<Option<T>>,
    iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>,
    iter_finished: bool
}

impl<T> Iterator for Interleave<T> {
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.buf.len() > 0 {
                let ret = self.buf.pop_front().unwrap();
                return Some(Ok(ret));
            }

            assert_eq!(0, self.buf2.len());

            for i in self.iter_vec.iter_mut() {
                match i.next() {
                    None => {
                        self.buf2.push_back(None);
                    },
                    Some(v) => {
                        match v {
                            Ok(ok_v) => {
                                self.buf2.push_back(Some(ok_v));
                            },
                            Err(err_v) => { // upstream error
                                self.iter_finished = true;
                                return Some(Err(err_v));
                            }
                        }
                    }
                }
            }

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

pub fn interleave<T>(iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where T: 'static
{
    Box::new(Interleave {
        buf: VecDeque::new(),
        buf2: VecDeque::new(),
        iter_vec,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {

    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let mut v = Vec::new();
        v.push(generate_okok_iterator(vec![1,2,3]));
        v.push(generate_okok_iterator(vec![4,5]));
        v.push(generate_okok_iterator(vec![6,7,8]));

        let ret = interleave(v).collect::<Vec<_>>();
        assert_eq!(vec![1, 4, 6, 2, 5, 7], extract_value_from_result_vec(ret).0);


        let mut v = Vec::new();
        v.push(generate_okok_iterator(vec![1,2,3]));
        v.push(generate_okokerr_iterator(vec![4], error::overflow_error("[test]".to_string())));
        v.push(generate_okok_iterator(vec![6,7,8]));

        let ret = interleave(v).collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![1, 4, 6], ret2.0);
        assert_eq!(error::Kind::OverflowError, ret2.1.unwrap().kind());
    }
}