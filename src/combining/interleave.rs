use std::collections::VecDeque;


pub struct Interleave<T> {
    buf: VecDeque<T>,
    buf2: VecDeque<Option<T>>,
    iter_vec: Vec<Box<dyn Iterator<Item = T>>>,
    iter_finished: bool
}

impl<T> Iterator for Interleave<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.buf.len() > 0 {
                let ret = self.buf.pop_front().unwrap();
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

pub fn interleave<T>(iter_vec: Vec<Box<dyn Iterator<Item = T>>>) -> Box<dyn Iterator<Item = T>> 
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
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut v = Vec::new();
        v.push(iter_from_vec(vec![1,2,3]));
        v.push(iter_from_vec(vec![4,5]));
        v.push(iter_from_vec(vec![6,7,8]));

        let ret = interleave(v).collect::<Vec<_>>();
        assert_eq!(vec![1, 4, 6, 2, 5, 7], ret);
    }
}