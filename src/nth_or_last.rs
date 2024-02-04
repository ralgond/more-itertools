use crate::error::Error;
use crate::error;
use std::collections::LinkedList;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#nth_or_last
pub fn nth_or_last<I>(iterable: I, n: usize, default: Option<I::Item>) -> Result<I::Item, Error>
where
    I: IntoIterator,
{
    let mut buf = LinkedList::new();
    
    let mut iter = iterable.into_iter();
    
    let mut i = 0;

    let mut ret = iter.next();
    loop {
        if i == n {
            match ret {
                Some(v) => { return Ok(v); }
                None => { break; } 
            }
        }
        
        match ret {
            Some(v) => {
                if buf.len() == 0 {
                    buf.push_back(v);
                } else {
                    buf.pop_front();
                    buf.push_back(v);
                }
            },
            None => { break; }
        }
        ret = iter.next();
        i += 1;
    }  

    if buf.len() > 0 {
        return Ok(buf.pop_front().unwrap()); // it must not panics!!!!
    } else {
        match default {
            Some(default_value) => { return Ok(default_value); }
            None => {
                return Err(error::value_error(String::from("nth_or_last() was called on an empty iterable, and no default was provided.")));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ret = nth_or_last(vec![0,1,2,3], 2, Some(5));
        match ret {
            Ok(v) => { assert_eq!(2, v) },
            Err(_) => { assert!(false) }
        }

        let ret2 = nth_or_last(vec![0,1], 2, Some(5));
        match ret2 {
            Ok(v) => { assert_eq!(1, v) },
            Err(_) => { assert!(false) }
        }

        let ret3 = nth_or_last(vec![], 0, Some(5));
        match ret3 {
            Ok(v) => { assert_eq!(5, v) },
            Err(_) => { assert!(false) }
        }

        let ret4 = nth_or_last(vec![], 0, None::<i32>);
        match ret4 {
            Ok(_) => { assert!(false) },
            Err(_) => { assert!(true) }
        }
    }
}