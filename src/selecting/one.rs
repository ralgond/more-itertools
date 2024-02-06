use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn one<I>(iterable: I) -> Result<I::Item, Error> 
where 
I: IntoIterator,
I::Item: Clone
{
    let result;
    let ret = spy(iterable, 2);
    match ret {
        None => { return Err(error::value_error("too short".to_string())); },
        Some(vec) => {
            if vec.len() > 1 {
                return Err(error::value_error("too long".to_string()));
            } else if vec.len() == 0 {
                return Err(error::value_error("too short".to_string()));
            } else {
                result = vec[0].clone();
            }
        }
    }

    return Ok(result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v1: Vec<String>= Vec::new();
        let ret1 = one(v1);
        match ret1 {
            Err(e) => { assert_eq!(*(e.message().unwrap()), String::from("too short")); },
            Ok(_) => { assert!(false); }
        }

        let v1: Vec<String>= vec!["too".to_string(), "many".to_string()];
        let ret1 = one(v1);
        match ret1 {
            Err(e) => { assert_eq!(*(e.message().unwrap()), String::from("too long")); },
            Ok(_) => { assert!(false); }
        }

        let v1: Vec<String>= vec!["too".to_string()];
        let ret1 = one(v1);
        match ret1 {
            Err(_) => { assert!(false); },
            Ok(v) => { assert_eq!(v, "too".to_string()); }
        }
    }
}