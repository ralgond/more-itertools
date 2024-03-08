use crate::error::Error;
use crate::error;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#first
pub fn first<T>(iter: &mut Box<dyn Iterator<Item = Result<T,Error>>>, default: Option<T>) -> Result<T, Error>
where
T: 'static + Clone
{
    let _next = iter.next();
    if let Some(v_next) = _next {
        match v_next {
            Ok(ok_v_next) => {
                return Ok(ok_v_next.clone());
            },
            Err(err_v_next) => {
                return Err(err_v_next);
            }
        }
    } else {
        if let Some(v_default) = default {
            return Ok(v_default.clone());
        } else {
            return Err(error::value_error(String::from("[first:empty iterable, and no default value]")));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::generate_okok_iterator;
    use crate::utils::generate_okokerr_iterator;

    use super::*;

    #[test]
    fn test1() {
        let ret = first(&mut generate_okok_iterator(vec![2,3,4]), Some(5));
        assert_eq!(2, ret.ok().unwrap());

        let ret2 = first(&mut generate_okok_iterator(vec![]), Some(5));
        assert_eq!(5, ret2.ok().unwrap());

        let ret3 = first(&mut generate_okok_iterator(vec![]), None::<i32>);
        assert_eq!(error::Kind::ValueError, ret3.err().unwrap().kind());

        let v = vec![1,2,3];
        let ret4 = first(&mut generate_okok_iterator(v), Some(0));
        assert_eq!(1, ret4.ok().unwrap());

        let ret5 = first(&mut generate_okokerr_iterator(vec![], error::overflow_error("for test".to_string())), Some(5));
        assert_eq!(error::Kind::OverflowError, ret5.err().unwrap().kind());
    }
}

