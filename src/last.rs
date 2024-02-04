use std::collections::LinkedList;

use crate::error::Error;
use crate::error;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#last
pub fn last<I>(iterable: I, default: Option<I::Item>) -> Result<I::Item, Error>
where
    I: IntoIterator,
{
    let mut buf = LinkedList::new();
    
    let mut iter = iterable.into_iter();
    
    let mut ret = iter.next();
    loop {
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
    }  

    if buf.len() > 0 {
        return Ok(buf.pop_front().unwrap()); // it must not panics!!!!
    } else {
        match default {
            Some(default_value) => { return Ok(default_value); }
            None => {
                return Err(error::value_error(String::from("last() was called on an empty iterable, and no default was provided.")));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ret = last(vec![2,3,4], Some(5));
        match ret {
            Ok(v) => { assert_eq!(4, v) },
            Err(_) => { assert!(false) }
        }

        let ret2 = last(vec![], Some(5));
        match ret2 {
            Ok(v) => { assert_eq!(5, v) },
            Err(_) => { assert!(false) }
        }

        let ret2 = last(vec![], None::<i32>);
        match ret2 {
            Ok(_) => { assert!(false) },
            Err(_) => { assert!(true) }
        }
    }
}