use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn strictly_n<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, n: usize) -> Option<Result<Vec<T>, Error>>
where 
T: 'static
{
    if n == 0 {
        return Some(Err(error::value_error("[strictly_n:n should not be 0]".to_string())));
    }

    if n == usize::MAX {
        return Some(Err(error::value_error("[strictly_n:n should not be usize::MAX]".to_string())));
    }

    let ret = spy(iter, n+1);
    if let Some(v_ret) = ret {
        match v_ret {
            Ok(ok_v_ret) => {
                if ok_v_ret.len() > n {
                    return Some(Err(error::value_error("[strictly_n:too long]".to_string())));
                } else if ok_v_ret.len() < n {
                    return Some(Err(error::value_error("[strictly_n:too short]".to_string())));
                } else {
                    return Some(Ok(ok_v_ret));
                }
            },
            Err(err_v_ret) => { // upstream error
                return Some(Err(err_v_ret));
            }
        }
    } else {
        return Some(Err(error::value_error("[strictly_n:too short]".to_string())));
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4];
        let ret1 = strictly_n(generate_okok_iterator(v), 4);
        assert_eq!(ret1.unwrap().ok(), Some(vec![1,2,3,4]));

        let v = vec![1,2,3];
        let ret1 = strictly_n(generate_okok_iterator(v), 4);
        assert_eq!(ret1.unwrap().err().unwrap().kind(), error::Kind::ValueError);

        let v = vec![1,2,3,4,5];
        let ret1 = strictly_n(generate_okok_iterator(v), 4);
        assert_eq!(ret1.unwrap().err().unwrap().kind(), error::Kind::ValueError);
    }
}