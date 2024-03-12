use std::collections::VecDeque;

use crate::error::Error;


pub struct InterleaveLongest<T> {
    buf: VecDeque<T>,
    buf2: VecDeque<Option<T>>,
    iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>,
    iter_finished: bool,
    fillnone: bool,
    fillvalue: Option<T>
}

impl<T> Iterator for InterleaveLongest<T> 
where
T: Clone
{
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

            if self.buf2.iter().all(|x| match x {
                None => { return true; },
                Some(_) => { return false; }
            }) {
                self.iter_finished = true;
            } else {
                while self.buf2.len() > 0 {
                    match self.buf2.pop_front() {
                        None => {
                            
                        },
                        Some(v) => {
                            if !v.is_none() {
                                self.buf.push_back(v.unwrap());
                            } else {
                                if self.fillnone {
                                    self.buf.push_back(self.fillvalue.as_ref().unwrap().clone());
                                }
                            }
                        }
                    }
                }
            }
        }   
    }
}

pub fn interleave_longest<T>(iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>, fillvalue: Option<T>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where T: Clone + 'static
{
    let fillnone;
    match fillvalue {
        None => { fillnone = false },
        Some(_) => { fillnone = true }
    }

    Box::new(InterleaveLongest {
        buf: VecDeque::new(),
        buf2: VecDeque::new(),
        iter_vec,
        iter_finished: false,
        fillnone: fillnone,
        fillvalue
    })
}

#[cfg(test)]
mod tests {
    use crate::utils::{extract_value_from_result_vec, generate_okok_iterator};

    use super::*;

    #[test]
    fn test1() {
        let mut v = Vec::new();
        v.push(generate_okok_iterator(vec![1,2,3]));
        v.push(generate_okok_iterator(vec![4,5]));
        v.push(generate_okok_iterator(vec![6,7,8]));

        let ret = interleave_longest(v, None).collect::<Vec<_>>();
        assert_eq!(vec![1, 4, 6, 2, 5, 7, 3, 8], extract_value_from_result_vec(ret).0);
        //println!("{:?}", ret);


        let mut v = Vec::new();
        v.push(generate_okok_iterator(vec![1,2,3]));
        v.push(generate_okok_iterator(vec![4,5]));
        v.push(generate_okok_iterator(vec![6,7,8]));

        let ret = interleave_longest(v, Some(0)).collect::<Vec<_>>();
        assert_eq!(vec![1, 4, 6, 2, 5, 7, 3, 0, 8], extract_value_from_result_vec(ret).0);
        //println!("{:?}", ret);
    }
}