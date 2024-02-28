use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn only<T>(iter: &mut Box<dyn Iterator<Item = T>>, default: Option<T>) -> Result<T, Error> 
where
T: Clone + 'static
{
    let result;
    let ret = spy(iter, 2);
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
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v1: Vec<String>= Vec::new();
        let ret1 = only(&mut iter_from_vec(v1), Some("missing".to_string()));
        assert_eq!(ret1.ok().unwrap(), "missing".to_string());


        let v1: Vec<String>= vec!["too".to_string(), "many".to_string()];
        let ret1 = only(&mut iter_from_vec(v1), Some("missing".to_string()));
        assert_eq!(*ret1.err().unwrap().message().unwrap(), String::from("too long"));


        let v1: Vec<String>= vec!["too".to_string()];
        let ret1 = only(&mut iter_from_vec(v1), Some("missing".to_string()));
        assert_eq!(*ret1.ok().unwrap(), String::from("too"));
    }
}