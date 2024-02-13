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

}