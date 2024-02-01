use std::mem::swap;

use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Chunked<I: Iterator> {
    buf: Vec<I::Item>,
    n: usize,
    iter: I
}


impl<I: Iterator> Iterator for Chunked<I> {
    type Item = Result<Vec<<I as Iterator>::Item>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return Some(Err(error::value_error(String::from("n should not be 0"))));
        }

        let mut ret = Vec::new();
        while self.buf.len() < self.n {
            let ele = self.iter.next();
            match ele {
                Some(ele_value) => { self.buf.push(ele_value) }
                None => {
                    if self.buf.len() == 0 {
                        return None;
                    } else {
                        swap(&mut ret, &mut self.buf);
                        self.buf = Vec::new();
                        return Some(Ok(ret));
                    }
                }
            }

            if self.buf.len() == self.n {
                ret = Vec::new();
                swap(&mut ret, &mut self.buf);
                return Some(Ok(ret));
            }
        }
        
        // while self.visited_cnt < self.n {
        //     self.visited_cnt += 1;
        //     if self.buf.len() == self.n {
        //         swap(&mut ret, &mut self.buf);
        //         break;
        //     } else if self.visited_cnt == self.n {
        //         swap(&mut ret, &mut self.buf);
        //         break;
        //     }
        // }

        return Some(Ok(ret));

    }
}

pub fn chunked<I>(iterable: I, n: usize) -> Chunked<I::IntoIter>
where
    I: IntoIterator,
{
    Chunked {
        buf: Vec::new(),
        n: n,
        iter: iterable.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut it = chunked(vec![1,2,3,4,5,6,7,8,9,10], 3);
        // println!("{:?}", it.next());
        // println!("{:?}", it.next());
        // println!("{:?}", it.next());
        // println!("{:?}", it.next());
        // println!("{:?}", it.next());

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec![1,2,3]); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec![4,5,6]); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec![7,8,9]); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec![10]); },
            Err(_) => {}
        }

        match it.next() {
            Some(_) => { assert!(false) }
            None => { assert!(true) }
        }

        match it.next() {
            Some(_) => { assert!(false) }
            None => { assert!(true) }
        }
    }
}

