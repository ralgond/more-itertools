use std::fmt::Debug;

use super::sliding_window::sliding_windowed;
use crate::error::Error;

pub struct Triplewise<T>
where 
T: Clone + Debug + 'static
{
    iter: Box<dyn Iterator<Item=Result<Vec<T>, Error>>>
}

impl<T> Iterator for Triplewise<T> 
where 
T: Clone + Debug
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}

pub fn triplewise<T>(iter: Box<dyn Iterator<Item=T>>) -> Triplewise<T>
where
T: Clone + Debug
{
    let ret: Triplewise<_> = Triplewise {
        iter: sliding_windowed(iter, 3)
    };

    return ret;
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4];
        let mut pw = triplewise(iter_from_vec(v));

        match pw.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2, vec![0, 1, 2]);
                    }
                    Err(_) => {}
                }
            }
        }

        match pw.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2, vec![1, 2, 3]);
                    }
                    Err(_) => {}
                }
            }
        }

        match pw.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2, vec![2, 3, 4]);
                    }
                    Err(_) => {}
                }
            }
        }

        match pw.next() {
            None => { assert!(true); }
            Some(_) => {
                assert!(false);
            }
        }


    }
}