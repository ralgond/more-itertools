use crate::error::Error;

pub fn extract_value_from_result_vec<T>(vec: Vec<Result<T, Error>>) -> (Vec<T>, bool) {
    let mut ret_vec = Vec::new();
    for v in vec.into_iter() {
        match v {
            Err(_) => { return (ret_vec, true); },
            Ok(v2) => { ret_vec.push(v2); }
        }
    }
    return (ret_vec, false);
}

pub fn join_string_vec(v: &Vec<char>) -> String{
    return v.iter().collect();
}

pub fn join_char_vec_second_level(l: &Vec<Vec<char>>) -> Vec<String> {
    let mut ret = vec![];
    for item in l {
        ret.push(join_string_vec(item));
    }
    return ret;
}

pub fn any(v: &Vec<bool>) -> bool {
    for i in v.iter() {
        if *i {
            return true;
        }
    }
    return false;
}


pub fn any_result(v: &Result<Vec<bool>, Error>) -> bool {
    if v.is_err() {
        return false;
    }
    return any(&(v.as_ref().ok().unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error;

    #[test]
    fn test1() {
        let v = vec![Ok(4),Ok(3),Ok(3)];
        let a = extract_value_from_result_vec(v);
        assert!(!a.1);
        assert_eq!(vec![4,3,3], a.0);

        let v = vec![Ok(4),Err(error::any_error(error::Kind::OverflowError, "Overflow".to_string())),Ok(3)];
        let a = extract_value_from_result_vec(v);
        assert!(a.1);
        assert_eq!(vec![4], a.0);
    }

    #[test]
    fn test2() {
        let v = vec!['a', 'b', 'c'];
        assert_eq!(join_string_vec(&v), "abc".to_string());
    }

    #[test]
    fn test_any() {
        let v = vec![true,true];
        assert_eq!(true, any(&v));

        let v = vec![true,false];
        assert_eq!(true, any(&v));

        let v: Vec<bool> = vec![false,false];
        assert_eq!(false, any(&v));

    }
}