use crate::error::Error;

use crate::itertools::islice::islice;

pub fn nth<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, n: usize, default: Option<T>) -> Option<Result<T,Error>>
where
T: 'static
{
    let mut i = islice(iter, n, n+1, 1);

    match i.next() {
        None => {
            match default {
                Some(_) => { return Some(Ok(default.unwrap())); }
                None => { return None; }
            }
        },
        Some(v) => {
            match v {
                Ok(ok_v) => { return Some(Ok(ok_v)); }
                Err(err_v) => { 
                    return Some(Err(err_v));
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{error, utils::{generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Some(Ok(5)), nth(generate_okok_iterator(vec![0,1,2,3,4,5]), 5, Some(1)));
        assert_eq!(Some(Ok(0)), nth(generate_okok_iterator(vec![0,1,2,3,4,5]), 7, Some(0)));
        assert_eq!(None, nth(generate_okok_iterator(vec![0,1,2,3,4,5]), 7, None));
    }

    #[test]
    fn test2() {
        assert_eq!(error::Kind::OverflowError, nth(generate_okokerr_iterator(
            vec![0,1,2,3,4,5], error::overflow_error("[test]".to_string())), 6, Some(1)).unwrap().err().unwrap().kind());
    }
}