use std::collections::VecDeque;
use std::fmt::Debug;
use crate::error::Error;
use crate::error;

pub struct Windowed<T>
where 
T: Clone + Debug + 'static
 {
    buf: VecDeque<T>,
    iter: Box<dyn Iterator<Item=Result<T,Error>>>,
    n: usize,
    step: usize,
    cache_first_window: bool,
    iter_finished: bool
}

impl<T> Iterator for Windowed<T> 
where 
T: Clone + Debug + 'static
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if self.n == 0 {
            return Some(Err(error::value_error(String::from("[windowed:n should not be 0]"))));
        }

        if !self.cache_first_window {
            for _ in 0..self.n {
                match self.iter.next() {
                    Some(v) => {
                        match v {
                            Ok(ok_v) => {
                                self.buf.push_back(ok_v);
                            },
                            Err(err_v) => { // upstream error
                                self.iter_finished = true;
                                return Some(Err(err_v));
                            }
                        }
                        
                    },
                    None => {
                        break;
                    }
                }
            }
            self.cache_first_window = true;

            if self.buf.len() < self.n {
                return None;
            }
            let mut ret = Vec::new();
            for v in self.buf.iter() {
                ret.push(v.clone());
            }
            return Some(Ok(ret));
        }

        

        for _ in 0..self.step {
            self.buf.pop_front();
            match self.iter.next() {
                Some(v) => {
                    match v {
                        Ok(ok_v) => {
                            self.buf.push_back(ok_v);
                        },
                        Err(err_v) => { // upstream error
                            self.iter_finished = true;
                            return Some(Err(err_v));
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        if self.buf.len() < self.n {
            return None;
        }
        let mut ret = Vec::new();
        for v in self.buf.iter() {
            ret.push(v.clone());
        }
        return Some(Ok(ret));
    }
}

/// https://more-itertools.readthedocs.io/en/v10.2.0/api.html#more_itertools.windowed
pub fn windowed<T>(iter: Box<dyn Iterator<Item=Result<T,Error>>>, n: usize, step: usize) -> Box<dyn Iterator<Item=Result<Vec<T>, Error>>> 
where
T: Clone + Debug + 'static
{
    Box::new(Windowed {
        buf: VecDeque::new(),
        iter,
        n,
        step,
        cache_first_window: false,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let mut w = windowed(generate_okok_iterator(v), 3, 1);

        assert_eq!(vec![1,2,3], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![2,3,4], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![3,4,5], w.next().unwrap().ok().unwrap());
        assert_eq!(None, w.next());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3,4,5,6,7,8];
        let mut w = windowed(generate_okok_iterator(v), 3, 2);
        assert_eq!(vec![1,2,3], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![3,4,5], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![5,6,7], w.next().unwrap().ok().unwrap());
        assert_eq!(None, w.next());
    }

    #[test]
    fn test3() {
        let v = vec![1,2];
        let mut w = windowed(generate_okok_iterator(v), 3, 1);
        assert_eq!(None, w.next());

        let v = vec![1,2];
        let mut w = windowed(generate_okok_iterator(v), 0, 1);
        assert_eq!(error::Kind::ValueError, w.next().unwrap().err().unwrap().kind());
    }
}