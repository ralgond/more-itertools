use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn strictly_n<I>(iterable: I, n: usize) -> Result<Vec<I::Item>, Error> 
where 
I: IntoIterator,
I::Item: Clone
{
    if n == 0 {
        return Err(error::value_error("n should not be 0".to_string()));
    }

    if n == usize::MAX {
        return Err(error::value_error("n should not be usize::MAX".to_string()));
    }

    let ret = spy(iterable, n+1);
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
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4];
        let ret1 = strictly_n(v, 4);
        match ret1 {
            Err(_) => { assert!(false); },
            Ok(ret) => { assert_eq!(vec![1,2,3,4], ret); }
        }

        let v = vec![1,2,3];
        let ret1 = strictly_n(v, 4);
        match ret1 {
            Err(_) => { assert!(true); },
            Ok(_) => { assert!(false); }
        }

        let v = vec![1,2,3,4,5];
        let ret1 = strictly_n(v, 4);
        match ret1 {
            Err(_) => { assert!(true); },
            Ok(_) => { assert!(false); }
        }
    }
}