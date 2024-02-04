use crate::error::Error;
use crate::error;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#first
pub fn first<I>(iterable: I, default: Option<I::Item>) -> Result<I::Item, Error>
where
    I: IntoIterator,
{
    let ret = iterable.into_iter().next();
    match ret {
        Some(first) => {
            return Ok(first);
        },
        None => {
            match default {
                Some(default_value) => { return Ok(default_value); }
                None => {
                    return Err(error::value_error(String::from("first() was called on an empty iterable, and no default value was provided.")));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ret = first(vec![2,3,4], Some(5));
        match ret {
            Ok(v) => { assert_eq!(2, v) },
            Err(_) => { assert!(false) }
        }

        let ret2 = first(vec![], Some(5));
        match ret2 {
            Ok(v) => { assert_eq!(5, v) },
            Err(_) => { assert!(false) }
        }

        let ret3 = first(vec![], None::<i32>);
        match ret3 {
            Ok(_) => { assert!(false) },
            Err(_) => { assert!(true) }
        }

        let v = vec![1,2,3];
        let ret4 = first(v, Some(0));
        match ret4 {
            Ok(v) => { assert_eq!(1, v) },
            Err(_) => { assert!(false) }
        }

        // println!("{:?}", v);
    }
}

