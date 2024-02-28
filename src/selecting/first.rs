use crate::error::Error;
use crate::error;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#first
pub fn first<T>(iter: &mut Box<dyn Iterator<Item = T>>, default: Option<T>) -> Result<T, Error>
where
T: 'static + Clone
{
    let ret = iter.next();
    match ret {
        Some(first) => {
            return Ok(first.clone());
        },
        None => {
            match default {
                Some(default_value) => { return Ok(default_value.clone()); }
                None => {
                    return Err(error::value_error(String::from("first() was called on an empty iterable, and no default value was provided.")));
                }
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
        let ret = first(&mut iter_from_vec(vec![2,3,4]), Some(5));
        assert_eq!(2, ret.ok().unwrap());

        let ret2 = first(&mut iter_from_vec(vec![]), Some(5));
        assert_eq!(5, ret2.ok().unwrap());

        let ret3 = first(&mut iter_from_vec(vec![]), None::<i32>);
        assert_eq!(error::Kind::ValueError, ret3.err().unwrap().kind());

        let v = vec![1,2,3];
        let ret4 = first(&mut iter_from_vec(v), Some(0));
        assert_eq!(1, ret4.ok().unwrap());

    }
}

