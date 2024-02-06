use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn only<I>(iterable: I, default: Option<I::Item>) -> Result<I::Item, Error> 
where 
I: IntoIterator,
I::Item: Clone
{
    let result;
    let ret = spy(iterable, 2);
    match ret {
        None => {
            match default {
                None => { return Err(error::value_error("too short and default is None".to_string())); }
                Some(v) => { return Ok(v); }
            }
        },
        Some(vec) => {
            if vec.len() > 1 {
                return Err(error::value_error("too long".to_string()));
            } else if vec.len() == 0 {
                match default {
                    None => { return Err(error::value_error("too short and default is None".to_string())); }
                    Some(v) => { return Ok(v); }
                }
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
        let ret1 = only(v1, Some("missing".to_string()));
        match ret1 {
            Err(_) => { assert!(false) },
            Ok(_) => { assert!(true); }
        }

        let v1: Vec<String>= vec!["too".to_string(), "many".to_string()];
        let ret1 = only(v1, Some("missing".to_string()));
        match ret1 {
            Err(e) => { assert_eq!(*(e.message().unwrap()), String::from("too long")); },
            Ok(_) => { assert!(false); }
        }

        let v1: Vec<String>= vec!["too".to_string()];
        let ret1 = only(v1, Some("missing".to_string()));
        match ret1 {
            Err(_) => { assert!(false); },
            Ok(v) => { assert_eq!(v, "too".to_string()); }
        }
    }
}