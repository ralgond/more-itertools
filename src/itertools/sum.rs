

#[macro_export]
macro_rules! sum {
    ($arg:ty,  $iter:expr, $init:expr) => {
        {
            let mut overflow = false;
            let mut upstream_error = false;
            let mut the_upstream_error = Err(error::any_error(error::Kind::None, "none error".to_string()));
            let mut sum0: $arg = $init;

            let mut _next = $iter.next();
            while !_next.is_none() {
                match _next {
                    None=>{
                        break;
                    }
                    Some(v) => {
                        if !v.is_ok() {
                            upstream_error = true;
                            the_upstream_error = v;
                            break;
                        } else {
                            let add_result = <$arg>::overflowing_add(sum0, v.ok().unwrap());
                        
                            if add_result.1 {
                                overflow = true;
                                break;
                            } else {
                                sum0 = add_result.0;
                                _next = $iter.next();
                                continue;
                            }
                        }
                    }
                }
            }

            if upstream_error {
                the_upstream_error
            } else if overflow {
                Err(error::any_error(error::Kind::OverflowError, "Add overflow.".to_string()))
            } else {
                Ok(sum0)
            }
        }
    };
}

pub(crate) use sum;


#[cfg(test)]
mod tests {
    use crate::error;
    use crate::itertools::iter::iter_from_vec;
    use crate::itertools::map::map2_result;

    //use super::*;

    #[test]
    fn test1() {
        let v = vec![Ok(1usize),Ok(2),Ok(3)];
        let mut iter: Box<dyn Iterator<Item = Result<usize, error::Error>>> = iter_from_vec(v);
        let ret = sum!(usize, iter, 0);
        assert_eq!(Ok(6), ret);

        let v = Vec::<Result<usize,error::Error>>::new();
        let mut iter = iter_from_vec(v);
        let ret = sum!(usize, iter, 0);
        assert_eq!(Ok(0), ret);

        let v = vec![Ok(usize::MAX),Ok(2),Ok(3)];
        let mut iter = iter_from_vec(v);
        let ret = sum!(usize, iter, 0);
        assert_eq!(error::Kind::OverflowError, ret.err().unwrap().kind());

        let v1 = iter_from_vec(vec![2, 3, usize::MAX]);
        let v2 = iter_from_vec(vec![1, 2, 3]);
        let mut iter = map2_result(v1, v2, |x, y| {
            let ret = x.overflowing_mul(y);
                if ret.1 {
                    return Err(error::any_error(error::Kind::OverflowError, "multiple overflow.".to_string()));
                } else {
                    return Ok(ret.0);
                }
        });
        let ret = sum!(usize, iter, 0);
        assert!(ret.is_err());
        assert!(ret.err().unwrap().message().unwrap().contains("multiple overflow"));

    }
}