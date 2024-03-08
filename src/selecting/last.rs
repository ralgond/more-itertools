use crate::error::Error;
use crate::error;
use crate::others::cache_last::cache_last;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#last
pub fn last<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, default: Option<T>) -> Result<T, Error>
where
T: Clone + 'static
{
    let mut cl = cache_last(iter);
    let mut cl_iter = cl.iter();
    loop {
        if let Some(v_next) = cl_iter.next() {
            match v_next {
                Ok(_) => {
                    continue;
                },
                Err(err_v_next) => {
                    return Err(err_v_next); // upstream error
                }
            }
        } else {
            if cl.is_empty() {
                match default {
                    Some(default_value) => { return Ok(default_value); }
                    None => {
                        return Err(error::value_error(String::from("[last:empty iterable, and no default]")));
                    }
                }
            } else {
                return cl.get_last_item().unwrap();
            }
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
        let ret = last(generate_okok_iterator(vec![2,3,4]), Some(5));
        assert_eq!(4, ret.ok().unwrap());

        let ret2 = last(generate_okok_iterator(vec![]), Some(5));
        assert_eq!(5, ret2.ok().unwrap());

        let ret2 = last(generate_okok_iterator(vec![]), None::<i32>);
        assert_eq!(error::Kind::ValueError, ret2.err().unwrap().kind());

        let ret2 = last(generate_okokerr_iterator(vec![2,3,4],error::overflow_error("[test]".to_string())), Some(5));
        assert_eq!(error::Kind::OverflowError, ret2.err().unwrap().kind());
    }
}