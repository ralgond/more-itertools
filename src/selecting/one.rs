use crate::error::Error;
use crate::error;
use crate::look_ahead_back::spy::spy;

pub fn one<T>(iter: &mut Box<dyn Iterator<Item = T>>) -> Result<T, Error> 
where 
T: Clone + 'static
{
    let result;
    let ret = spy(iter, 2);
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
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut v1 = iter_from_vec(Vec::<i32>::new());
        let ret1 = one(&mut v1);
        assert_eq!(*ret1.err().unwrap().message().unwrap(), String::from("too short"));

        let mut v1 = iter_from_vec(vec!["too".to_string(), "many".to_string()]);
        let ret1 = one(&mut v1);
        assert_eq!(*ret1.err().unwrap().message().unwrap(), String::from("too long"));

        let mut v1 = iter_from_vec(vec!["too".to_string()]);
        let ret1 = one(&mut v1);
        assert_eq!(*ret1.ok().unwrap(), String::from("too"));
    }
}