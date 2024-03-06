use crate::error::{self, Error};

pub fn ilen<T>(mut iter: Box<dyn Iterator<Item = Result<T, Error>>>, init: usize) -> Result<usize, Error>  {
    let mut ret = init;
    loop {
        let _next = iter.next();
        if let Some(upstream_res) = _next {
            if upstream_res.is_err() {
                return Err(upstream_res.err().unwrap());
            } else {
                let (add_res, overflow) = usize::overflowing_add(ret, 1);
                if overflow {
                    return Err(error::overflow_error("[ilen] add overflow".to_string()));
                } else {
                    ret = add_res;
                    continue;
                }
            }
        } else {
            return Ok(ret);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error, utils::{generate_okok_iterator, generate_okokerr_iterator}};

    #[test]
    fn test1() {
        let iter = generate_okok_iterator(vec![1,2,3]);
        assert_eq!(3, ilen(iter, 0).ok().unwrap());

        let iter = generate_okok_iterator(vec![1,2,3]);
        assert_eq!(error::Kind::OverflowError, ilen(iter, usize::MAX).err().unwrap().kind());

        let iter = generate_okokerr_iterator(vec![1,2,3], error::value_error("for test".to_string()));
        assert_eq!(error::Kind::ValueError, ilen(iter, 0).err().unwrap().kind());
    }
}