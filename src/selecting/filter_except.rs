use crate::error::Error;
use crate::error;


pub struct FilterExcept<T> {
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    validator: fn(item: &T) -> Result<bool, Error>,
    acceptable_except: Vec<error::Kind>,
    error: Option<Error>,
    iter_finished: bool
}

impl<T> Iterator for FilterExcept<T> 
where
T: Clone + 'static
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            let ret = self.iter.next();
            if let Some(v) = ret {
                if v.is_err() {
                    self.error = Some(v.as_ref().err().unwrap().clone());
                    self.iter_finished = true;
                    return Some(v);
                }

                let val_result = (self.validator)(v.as_ref().ok().unwrap());
                match val_result {
                    Ok(ok_val_result) => {
                        if ok_val_result {
                            return Some(v);
                        } else {
                            continue;
                        }
                    },
                    Err(err_val_result) => {
                        if self.acceptable_except.contains(&err_val_result.kind()) {
                            continue;
                        } else {
                            self.error = Some(error::any_error(err_val_result.kind(), "[filter_except] ".to_string() + err_val_result.message().unwrap()));
                            self.iter_finished = true;
                        }
                    }
                }
            } else {
                self.iter_finished = true;
                return None
            }
        }
    }

}


pub fn filter_except<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, 
                        validator: fn(item: &T) -> Result<bool, Error>,
                        acceptable_except: Vec<error::Kind>) -> Box<dyn Iterator<Item = Result<T, Error>>>
where
T: Clone + 'static
{
                            
    Box::new(FilterExcept {
        // cur: 0,
        iter,
        validator,
        acceptable_except,
        error: None,
        iter_finished: false
    })
}


#[cfg(test)]
mod tests {
    use crate::utils::generate_okok_iterator;
    use crate::utils::generate_okokerr_iterator;
    use super::*;

    #[test]
    fn test1() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fe = filter_except(generate_okok_iterator(iterable), 
            |x| { 
                let ret  = x.parse::<i32>();
                match ret {
                    Ok(_) => { return Ok(true); }
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }, 
            vec![error::Kind::ValueError]);

        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("1", ok_v)
            }
        }

        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("2", ok_v)
            }
        }

        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("4", ok_v)
            }
        }

        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("5", ok_v)
            }
        }
        
        assert_eq!(None, fe.next());
        assert_eq!(None, fe.next());
    }

    #[test]
    fn test2() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fe = filter_except(generate_okok_iterator(iterable), 
            |x| { 
                let ret  = x.parse::<i32>();
                match ret {
                    Ok(_) => { return Ok(true); }
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }, 
            vec![]);

        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("1", ok_v)
            }
        }
        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("2", ok_v)
            }
        }
        if let Some(v) = fe.next() {
            if let Err(err_v) = v {
                assert_eq!(error::Kind::ValueError, err_v.kind());
            }
        }
        assert_eq!(None, fe.next());
        assert_eq!(None, fe.next());
    }

    #[test]
    fn test3() {
        let iterable = vec!["1", "2"];
        let mut fe = filter_except(generate_okokerr_iterator(iterable, error::overflow_error("for test".to_string())), 
            |x| { 
                let ret  = x.parse::<i32>();
                match ret {
                    Ok(_) => { return Ok(true); }
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }, 
            vec![]);

        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("1", ok_v)
            }
        }
        if let Some(v) = fe.next() {
            if let Ok(ok_v) = v {
                assert_eq!("2", ok_v)
            }
        }
        if let Some(v) = fe.next() {
            if let Err(err_v) = v {
                println!("{:?}", err_v);
                assert_eq!(error::Kind::OverflowError, err_v.kind());
            }
        }
        assert_eq!(None, fe.next());
        assert_eq!(None, fe.next());
    }
}