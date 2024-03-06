use crate::error;
use crate::error::Error;

struct Zip<T0, T1> {
    iter0: Box<dyn Iterator<Item=Result<T0,Error>>>,
    iter1: Box<dyn Iterator<Item=Result<T1,Error>>>,
    iter_finished: bool,
    iter_error: Option<Error>
}

impl<T0,T1> Iterator for Zip<T0,T1>
where
T0: Clone,
T1: Clone
{
    type Item = Result<(T0,T1), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if self.iter_error.is_some() {
            return Some(Err(self.iter_error.as_ref().unwrap().clone()));
        }

        let _next0 = self.iter0.next();
        let _next1 = self.iter1.next();
        match (&_next0, &_next1) {
            (Some(v0), Some(v1)) => {
                match (v0, v1) {
                    (Ok(v0_t), Ok(v1_t)) => {
                        return Some(Ok((v0_t.clone(), v1_t.clone())));
                    }, 
                    _ => {
                        if v0.is_err() {
                            return Some(Err(error::any_error(v0.as_ref().err().unwrap().kind(), 
                                            "[zip.v0] ".to_string()+v0.as_ref().err().unwrap().message().unwrap())));
                        } else {
                            return Some(Err(error::any_error(v0.as_ref().err().unwrap().kind(), 
                                            "[zip.v1] ".to_string()+v0.as_ref().err().unwrap().message().unwrap())));
                        }
                    }
                }
            },
            _=> { 
                self.iter_finished = true;
                return None;
            }
        }
    }
}

pub fn zip<T0: 'static,T1: 'static>(iter0: Box<dyn Iterator<Item=Result<T0,Error>>>, 
                                    iter1: Box<dyn Iterator<Item=Result<T1,Error>>>) -> Box<dyn Iterator<Item=Result<(T0,T1),Error>>> 
where
T0: Clone,
T1: Clone                                    
{
    Box::new(Zip{iter0,iter1,iter_finished:false, iter_error:None})
}

#[cfg(test)]
mod tests {
    use crate::{utils::{extract_value_from_result_vec, generate_okok_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let ret = zip(generate_okok_iterator(vec![1,2,3]), generate_okok_iterator(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let v = vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())];
        assert_eq!(v, extract_value_from_result_vec(ret.collect::<Vec<_>>()).0);


        let ret = zip(generate_okok_iterator(vec![1,2,3,4]), generate_okok_iterator(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let v = vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())];
        assert_eq!(v, extract_value_from_result_vec(ret.collect::<Vec<_>>()).0);

        let ret = zip(generate_okok_iterator(vec![1,2,3]), generate_okok_iterator(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]));
        let v = vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())];
        assert_eq!(v, extract_value_from_result_vec(ret.collect::<Vec<_>>()).0);
    }
}
