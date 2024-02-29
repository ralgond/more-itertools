use std::arch::x86_64::_MM_MASK_INEXACT;
use std::collections::VecDeque;
use std::fmt::Debug;
use crate::utils::are_same;

pub struct GroupBy<T> 
where T: Clone + Debug + PartialEq
{
    cur_key: Option<T>,
    buf: VecDeque<T>,
    emitted_first: bool,
    iter: Box<dyn Iterator<Item = T>>,
    iter_finished: bool,
    stop_next: bool,
}


impl<T> GroupBy<T> 
where T: Clone + Debug + PartialEq + 'static
{
    pub fn next_group(&mut self) -> Option<T> {
        loop {
            if self.iter_finished {
                return None;
            }

            let _next;
            if self.buf.len() > 0 {
                _next = self.buf.pop_front();
            } else {
                _next = self.iter.next();
            }
            
            match _next {
                None => {
                    self.iter_finished = true;
                    continue;
                },
                Some(v) => {
                    if self.cur_key.is_none() || !are_same(Some(&v), Some(self.cur_key.as_mut().unwrap())) {
                        self.cur_key = Some(v.clone());
                        self.emitted_first = false;
                        self.stop_next = false;
                        return Some(v);
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}

impl<T> Iterator for GroupBy<T> 
where
T: PartialEq + Clone + Debug + 'static
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.stop_next {
                return None;
            }

            if self.cur_key.is_none() {
                return None;
            }

            if !self.emitted_first {
                self.emitted_first = true;
                return Some(self.cur_key.as_ref().unwrap().clone());
            }

            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    continue;
                },
                Some(v) => {
                    if !are_same(Some(&v), Some(self.cur_key.as_mut().unwrap())) {
                        self.buf.push_back(v.clone());
                        self.stop_next = true;
                        continue;
                    } else {
                        return Some(v);
                    }
                }
            }
        }
    }
}

pub fn groupby<T>(iter: Box<dyn Iterator<Item = T>>) -> GroupBy<T> 
where
T: PartialEq + Clone + Debug + 'static
{
    return GroupBy {
        cur_key: None,
        buf: VecDeque::new(),
        emitted_first: false,
        iter,
        iter_finished: false,
        stop_next: true
    }
}

mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut ret = groupby(iter_from_vec("AAAAbbbcccC".chars().collect::<Vec<char>>()));
        assert_eq!('A', ret.next_group().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('b', ret.next_group().unwrap());
        assert_eq!('b', ret.next().unwrap());
        assert_eq!('b', ret.next().unwrap());
        assert_eq!('b', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('c', ret.next_group().unwrap());
        assert_eq!('c', ret.next().unwrap());
        assert_eq!('c', ret.next().unwrap());
        assert_eq!('c', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('C', ret.next_group().unwrap());
        assert_eq!('C', ret.next().unwrap());
        assert_eq!(None, ret.next());
    }

    #[test]
    fn test2() {
        let mut ret = groupby(iter_from_vec("0AAAAbbbcccCC".chars().collect::<Vec<char>>()));
        assert_eq!('0', ret.next_group().unwrap());
        assert_eq!('0', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('A', ret.next_group().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!('A', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('b', ret.next_group().unwrap());
        assert_eq!('b', ret.next().unwrap());
        assert_eq!('b', ret.next().unwrap());
        assert_eq!('b', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('c', ret.next_group().unwrap());
        assert_eq!('c', ret.next().unwrap());
        assert_eq!('c', ret.next().unwrap());
        assert_eq!('c', ret.next().unwrap());
        assert_eq!(None, ret.next());

        assert_eq!('C', ret.next_group().unwrap());
        assert_eq!('C', ret.next().unwrap());
        assert_eq!('C', ret.next().unwrap());
        assert_eq!(None, ret.next());
    }
}