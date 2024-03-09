use crate::error::Error;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#spy
pub fn spy<T>(mut iter: Box<dyn Iterator<Item = Result<T,Error>>>, n: usize) -> Option<Result<Vec<T>,Error>> 
where
T: 'static
{
    let mut ret = Vec::new();
    for _ in 0..n {
        if let Some(v) = iter.next() {
            match v {
                Ok(ok_v) => {
                    ret.push(ok_v);
                },
                Err(err_v) => { // upstream error
                    return Some(Err(err_v));
                }
            }
        } else {
            break;
        }
    }
    return Some(Ok(ret));
}

#[cfg(test)]
mod tests {
    

    use crate::{error, utils::{generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        assert_eq!(vec![1], spy(generate_okok_iterator(v), 1).unwrap().ok().unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(Vec::<i32>::new(), spy(generate_okok_iterator(v), 0).unwrap().ok().unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(vec![1,2,3], spy(generate_okok_iterator(v), 3).unwrap().ok().unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(vec![1,2,3,4,5], spy(generate_okok_iterator(v), 7).unwrap().ok().unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(error::Kind::OverflowError, spy(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), 7).unwrap().err().unwrap().kind());
    }
}