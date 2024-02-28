use std::collections::LinkedList;

use crate::error::Error;
use crate::error;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#last
pub fn last<T>(iter: &mut Box<dyn Iterator<Item = T>>, default: Option<T>) -> Result<T, Error>
where
T: 'static
{
    let mut buf = LinkedList::new();
    
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
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let ret = last(&mut iter_from_vec(vec![2,3,4]), Some(5));
        assert_eq!(4, ret.ok().unwrap());

        let ret2 = last(&mut iter_from_vec(vec![]), Some(5));
        assert_eq!(5, ret2.ok().unwrap());

        let ret2 = last(&mut iter_from_vec(vec![]), None::<i32>);
        assert_eq!(error::Kind::ValueError, ret2.err().unwrap().kind());
    }
}