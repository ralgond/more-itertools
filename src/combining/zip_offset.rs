use std::collections::VecDeque;

use crate::selecting::take::take;


pub struct ZipOffset<T> {
    buf: VecDeque<T>,
    buf2: VecDeque<Option<T>>,
    iter_vec: Vec<Box<dyn Iterator<Item = T>>>,
    iter_finished: bool,
    longest: bool,
    fillvalue: T
}

impl<T> Iterator for ZipOffset<T> 
where
T: Clone + 'static
{
    type Item = Vec<T>;

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
                return Some(ret);
            }

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

pub fn zip_offset<T>(mut iter_vec: Vec<Box<dyn Iterator<Item = T>>>,
            offsets_vec: Vec<usize>,
            longest: bool,
            fillvalue: T) -> Box<dyn Iterator<Item = Vec<T>>> 
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

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(iter_from_vec(v1));
        v.push(iter_from_vec(v2));

        let ret = zip_offset(v, vec![0,1], false, '9');
        assert_eq!(vec![vec!['0', 'b'], vec!['1', 'c'], vec!['2', 'd'], vec!['3', 'e']], ret.collect::<Vec<_>>());
    }

    #[test]
    fn test2() {
        let v1 = "0123".chars().collect::<Vec<_>>();
        let v2 = "abcdef".chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        v.push(iter_from_vec(v1));
        v.push(iter_from_vec(v2));

        let ret = zip_offset(v, vec![0,1], true, '9');
        assert_eq!(vec![vec!['0', 'b'], vec!['1', 'c'], vec!['2', 'd'], vec!['3', 'e'], vec!['9', 'f']], ret.collect::<Vec<_>>());
    }
}

