use crate::error::{self, Error};

struct Map<T, J> {
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    pred: fn(T) -> Result<J, Error>,
    iter_finished: bool,
    iter_error: Option<Error>
}

impl<T,J> Iterator for Map<T, J>
{
    type Item = Result<J, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if self.iter_error.is_some() {
            return Some(Err(self.iter_error.as_ref().unwrap().clone()));
        }

        let _next = self.iter.next();
        if let Some(v) = _next {
            if let Ok(v2) = v {
                let j = (self.pred)(v2);
                if j.is_err() {
                    self.iter_error = Some(j.as_ref().err().unwrap().clone());
                    self.iter_finished = true;
                }
                return Some(j);
            } else {
                self.iter_finished = true;
                return Some(Err(v.err().unwrap()));
            }
        } else {
            self.iter_finished = true;
            return None;
        }
    }
}

pub fn map<T: 'static, J: 'static>(iter: Box<dyn Iterator<Item=Result<T,Error>>>, pred: fn(T)->Result<J, Error>) -> Box<dyn Iterator<Item=Result<J, Error>>> 
{
    return Box::new(Map {
        iter,
        pred: pred,
        iter_finished: false,
        iter_error: None
    });
}


struct Map2<T0, T1, J> {
    iter0: Box<dyn Iterator<Item=Result<T0,Error>>>,
    iter1: Box<dyn Iterator<Item=Result<T1,Error>>>,
    pred: fn(&T0, &T1)->Result<J, Error>,
    iter_finished: bool,
    iter_error: Option<Error>
}

impl<T0, T1, J> Iterator for Map2<T0, T1, J>
{
    type Item = Result<J, Error>;

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
                        let j = (self.pred)(v0_t, v1_t);
                        if j.is_err() {
                            self.iter_error = Some(j.as_ref().err().unwrap().clone());
                            self.iter_finished = true;
                        }
                        return Some(j);
                    }, 
                    _ => {
                        if v0.is_err() {
                            self.iter_error = Some(error::any_error(v0.as_ref().err().unwrap().kind(), 
                            "[map.v0] ".to_string()+v0.as_ref().err().unwrap().message().unwrap()));

                            self.iter_finished = true;
                            return Some(Err(self.iter_error.as_ref().unwrap().clone()));
                        } else {
                            self.iter_error = Some(error::any_error(v1.as_ref().err().unwrap().kind(), 
                            "[map.v1] ".to_string()+v1.as_ref().err().unwrap().message().unwrap()));

                            self.iter_finished = true;
                            return Some(Err(self.iter_error.as_ref().unwrap().clone()));
                        }
                    }
                }
            },
            _ => {
                self.iter_finished = true;
                return None;
            },
        }
    }
}

pub fn map2<T0: 'static, T1: 'static, J: 'static>(
    iter0: Box<dyn Iterator<Item = Result<T0,Error>>>, 
    iter1: Box<dyn Iterator<Item = Result<T1,Error>>>,
    pred: fn(&T0,&T1)->Result<J,Error>) -> Box<dyn Iterator<Item = Result<J,Error>>> 
{
    return Box::new(Map2 {
        iter0,
        iter1,
        pred: pred,
        iter_finished: false,
        iter_error: None
    });
}


#[cfg(test)]
mod tests {
    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = generate_okok_iterator(vec![1,2,3]);
        let ret = map(v, |x| {Ok(x==3)});
        let ret = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert!(ret.1.is_none());
        assert_eq!(vec![false,false,true], ret.0);
    }

    #[test]
    fn test1_error() {
        let v = generate_okokerr_iterator(vec![1,2,3], error::overflow_error("for test".to_string()));
        let ret = map(v, |x| {Ok(x==3)});
        let ret = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert!(ret.1.is_some());
        assert_eq!(vec![false,false,true], ret.0);
    }

    #[test]
    fn test2() {
        let v0 = generate_okok_iterator(vec![1,2,3]);
        let v1 = generate_okok_iterator(vec![2,3,4]);
        let ret = map2(v0, v1, |x,y| {Ok(x*y)});
        let ret = extract_value_from_result_vec(ret.collect::<Vec<_>>());
        assert!(ret.1.is_none());
        assert_eq!(vec![2,6,12], ret.0);
    }

    #[test]
    fn test2_result() {
        let v0 = generate_okok_iterator(vec![1,2,i32::MAX]);
        let v1 = generate_okok_iterator(vec![2,3,4]);
        let mut ret = map2(
            v0, 
            v1, 
            |x,y| {
                let ret = x.overflowing_mul(*y);
                if ret.1 {
                    return Err(error::any_error(error::Kind::OverflowError, "multiple overflow.".to_string()));
                } else {
                    return Ok(ret.0);
                }
            }
        );
        assert_eq!(Ok(2), ret.next().unwrap());
        assert_eq!(Ok(6), ret.next().unwrap());
        assert_eq!(error::Kind::OverflowError, ret.next().unwrap().err().unwrap().kind());
        assert_eq!(None, ret.next());
    }
}

