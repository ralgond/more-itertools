use super::sliding_window::{sliding_windowed, SlidingWindow};
use crate::error::Error;

pub struct Pairwise<I>
where 
    I: IntoIterator,
    I::Item: Clone
{
    sliding_window: SlidingWindow<I::IntoIter>
}

impl<I> Iterator for Pairwise<I> 
where 
    I: Iterator,
    I::Item: Clone
{
    type Item = Result<Vec<<I as Iterator>::Item>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        return self.sliding_window.next();
    }
}

pub fn pairwise<I>(iterable: I) -> Pairwise<I::IntoIter>
where
    I: IntoIterator,
    I::Item: Clone
{
    let ret: Pairwise<_> = Pairwise {
        sliding_window: sliding_windowed(iterable, 2)
    };

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4];
        let mut pw = pairwise(v);

        match pw.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2, vec![0, 1]);
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
                        assert_eq!(v2, vec![1, 2]);
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
                        assert_eq!(v2, vec![2, 3]);
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
                        assert_eq!(v2, vec![3, 4]);
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