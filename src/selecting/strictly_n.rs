use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn strictly_n<T>(iter: &mut Box<dyn Iterator<Item = T>>, n: usize) -> Result<Vec<T>, Error> 
where 
T: 'static
{
    if n == 0 {
        return Err(error::value_error("n should not be 0".to_string()));
    }

    if n == usize::MAX {
        return Err(error::value_error("n should not be usize::MAX".to_string()));
    }

    let ret = spy(iter, n+1);
    match ret {
        None => { return Err(error::value_error("too short".to_string())); },
        Some(vec) => {
            if vec.len() > n {
                return Err(error::value_error("too long".to_string()));
            } else if vec.len() < n {
                return Err(error::value_error("too short".to_string()));
            } else {
                return Ok(vec);
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
        let v = vec![1,2,3,4];
        let ret1 = strictly_n(&mut iter_from_vec(v), 4);
        assert_eq!(ret1.ok(), Some(vec![1,2,3,4]));

        let v = vec![1,2,3];
        let ret1 = strictly_n(&mut iter_from_vec(v), 4);
        assert_eq!(ret1.err().unwrap().kind(), error::Kind::ValueError);

        let v = vec![1,2,3,4,5];
        let ret1 = strictly_n(&mut iter_from_vec(v), 4);
        assert_eq!(ret1.err().unwrap().kind(), error::Kind::ValueError);
    }
}