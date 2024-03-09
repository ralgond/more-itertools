use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn one<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>) -> Option<Result<T, Error>> 
where 
T: Clone + 'static
{
    let ret = spy(iter, 2);
    if let Some(v_ret) = ret {
        match v_ret {
            Ok(ok_v_ret) => {
                if ok_v_ret.len() > 1 {
                    return Some(Err(error::value_error("[one:too long]".to_string())));
                } else if ok_v_ret.len() == 0 {
                    return Some(Err(error::value_error("[one:too short]".to_string())));
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
        return Some(Err(error::value_error("[one:too short]".to_string())));
    }
}


#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v1 = generate_okok_iterator(Vec::<i32>::new());
        let ret1 = one(v1);
        assert_eq!(*ret1.unwrap().err().unwrap().message().unwrap(), String::from("[one:too short]"));

        let v1 = generate_okok_iterator(vec!["too".to_string(), "many".to_string()]);
        let ret1 = one(v1);
        assert_eq!(*ret1.unwrap().err().unwrap().message().unwrap(), String::from("[one:too long]"));

        let v1 = generate_okok_iterator(vec!["too".to_string()]);
        let ret1 = one(v1);
        assert_eq!(*ret1.unwrap().ok().unwrap(), String::from("too"));
    }
}