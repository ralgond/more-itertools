use std::fmt::Debug;

use super::windowed::windowed;
use crate::error::Error;

pub struct SlidingWindow<T>
where 
T: Clone + Debug + 'static
{
    iter: Box<dyn Iterator<Item=Result<Vec<T>, Error>>>
}

impl<T> Iterator for SlidingWindow<T> 
where 
T: Clone + Debug + 'static
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}

pub fn sliding_windowed<T>(iter: Box<dyn Iterator<Item=T>>, n: usize) -> Box<dyn Iterator<Item=Result<Vec<T>, Error>>> 
where
T: Clone + Debug + 'static
{
    let ret = windowed(iter, n, 1);

    return ret;
}


#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4,5];

        let mut w = sliding_windowed(iter_from_vec(v), 4);
        match w.next().unwrap() {
            Ok(v) => { assert_eq!(vec![0,1,2,3], v); }
            Err(_) => { assert!(false); }
        }
        match w.next().unwrap() {
            Ok(v) => { assert_eq!(vec![1,2,3,4], v); }
            Err(_) => { assert!(false); }
        }
        match w.next().unwrap() {
            Ok(v) => { assert_eq!(vec![2,3,4,5], v); }
            Err(_) => { assert!(false); }
        }
        match w.next() {
            Some(_) => { assert!(false); }
            None => { assert!(true); }
        }
    }

    #[test]
    fn test2() {
        let v = vec![0,1,2];
        let mut w = sliding_windowed(iter_from_vec(v), 4);
        match w.next() {
            Some(_) => { assert!(false); }
            None => { assert!(true); }
        }
    }
}
