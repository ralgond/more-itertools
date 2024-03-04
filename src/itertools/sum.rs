use crate::error;

#[macro_export]
macro_rules! sum {
    ($arg:ty,  $iter:expr, $init:expr) => {
        {
            let mut overflow = false;
            let mut sum0: $arg = $init;

            let mut _next = $iter.next();
            while !_next.is_none() {
                match _next {
                    None=>{
                        break;
                    }
                    Some(v) => {
                        let add_result = <$arg>::overflowing_add(sum0, v);
                        
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

            if overflow {
                Err(error::any_error(error::Kind::OverflowError, "Add overflow.".to_string()))
            } else {
                Ok(sum0)
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1usize,2,3];
        let mut iter = iter_from_vec(v);
        let ret = sum!(usize, iter, 0);
        assert_eq!(Ok(6), ret);

        let v = Vec::<usize>::new();
        let mut iter = iter_from_vec(v);
        let ret = sum!(usize, iter, 0);
        assert_eq!(Ok(0), ret);

        let v = vec![usize::MAX,2,3];
        let mut iter = iter_from_vec(v);
        let ret = sum!(usize, iter, 0);
        assert_eq!(error::Kind::OverflowError, ret.err().unwrap().kind());
    }
}