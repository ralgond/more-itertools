use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn only<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, default: Option<T>) -> Option<Result<T, Error>> 
where
T: Clone + 'static
{
    let ret = spy(iter, 2);
    if let Some(v_ret) = ret {
        match v_ret {
            Ok(ok_v_ret) => {
                if ok_v_ret.len() > 1 {
                    return Some(Err(error::value_error("[only:too long]".to_string())));
                } else if ok_v_ret.len() == 0 {
                    if let Some(v_default) = default {
                        return Some(Ok(v_default));
                    } else {
                        return Some(Err(error::value_error("[only:too short and no default]".to_string())));
                    }
                } else {
                    let result = ok_v_ret[0].clone();
                    return Some(Ok(result));
                }
            },
            Err(err_v_ret) => { // upstream error
                return Some(Err(err_v_ret));
            }
        }
    } else {
        if let Some(v_default) = default {
            return Some(Ok(v_default));
        } else {
            return Some(Err(error::value_error("[only:too short and no default]".to_string())));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v1: Vec<String>= Vec::new();
        let ret1 = only(generate_okok_iterator(v1), Some("missing".to_string()));
        assert_eq!(ret1.unwrap().ok().unwrap(), "missing".to_string());


        let v1: Vec<String>= vec!["too".to_string(), "many".to_string()];
        let ret1 = only(generate_okok_iterator(v1), Some("missing".to_string()));
        assert_eq!(*ret1.unwrap().err().unwrap().message().unwrap(), String::from("[only:too long]"));


        let v1: Vec<String>= vec!["too".to_string()];
        let ret1 = only(generate_okok_iterator(v1), Some("missing".to_string()));
        assert_eq!(*ret1.unwrap().ok().unwrap(), String::from("too"));
    }
}