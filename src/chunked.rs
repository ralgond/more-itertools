use std::mem::swap;

use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Chunked<I: Iterator> {
    buf: Vec<I::Item>,
    n: usize,
    iter: I,
    strict: bool
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
                        if self.strict && ret.len() < self.n {
                            return Some(Err(error::value_error(String::from("iterable is not divisible by n."))));
                        }
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

        return Some(Ok(ret));

    }
}

/// https://more-itertools.readthedocs.io/en/stable/_modules/more_itertools/more.html#chunked
pub fn chunked<I>(iterable: I, n: usize, strict: bool) -> Chunked<I::IntoIter>
where
    I: IntoIterator,
{
    Chunked {
        buf: Vec::new(),
        n: n,
        iter: iterable.into_iter(),
        strict: strict
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_no_strict() {
        let mut it = chunked(vec![1,2,3,4,5,6,7,8,9,10], 3, false);
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

    #[test]
    fn test2_strict() {
        let mut it = chunked(vec![1,2,3,4,5,6,7,8,9,10], 3, true);

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
            Ok(_) => { assert!(false) },
            Err(_) => { assert!(true)}
        }
    }

    #[test]
    fn test3_no_strict_chars() {
        let mut it = chunked("abcdefghij".chars(), 3, false);

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec!['a','b','c']); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec!['d', 'e', 'f']); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec!['g', 'h', 'i']); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec!['j']); },
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

    #[test]
    fn test3_no_strict_string() {
        let v: [String; 4] = [String::from("1"),
                            String::from("2"),
                            String::from("3"),
                            String::from("4")
                            ];

        let mut it = chunked(v, 3, false);

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec![String::from("1"), String::from("2") ,String::from("3")]); },
            Err(_) => {}
        }

        match it.next().unwrap() {
            Ok(value) => { assert_eq!(value, vec![String::from("4")]); },
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

