//use crate::itertools::sum::sum;

#[macro_export]
macro_rules! dotproduct {
    ($type0:ty, $iter0:expr, $iter1:expr, $init:expr) => {
        {
            let mut map_iter = map2($iter0, $iter1, |x, y| {
                let ret = <$type0>::overflowing_mul(*x, *y);
                    if ret.1 {
                        return Err(error::any_error(error::Kind::OverflowError, "multiple overflow.".to_string()));
                    } else {
                        return Ok(ret.0);
                    }
                });
            let sum_iter = sum!($type0, map_iter, $init);
            sum_iter
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::error;
    use crate::itertools::sum::sum;
    use crate::itertools::map::map2;
    use crate::utils::generate_okok_iterator;

    #[test]
    fn test1() {
        let iter_0 = generate_okok_iterator(vec![10i64,10]);
        let iter_1 = generate_okok_iterator(vec![20i64,20]);
        let ret = dotproduct!(i64, iter_0, iter_1, 0i64);
        assert_eq!(Ok(400i64), ret);


        let iter_0 = generate_okok_iterator(vec![10i64,10]);
        let iter_1 = generate_okok_iterator(vec![20i64,i64::MAX]);
        let ret = dotproduct!(i64, iter_0, iter_1, 0i64);
        assert!(ret.err().unwrap().message().unwrap().contains("multiple overflow"));
    }
}