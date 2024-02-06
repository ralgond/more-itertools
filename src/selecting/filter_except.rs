
use crate::error::Error;
use crate::error;


#[derive(Debug, Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterExcept<I: Iterator> {
    // cur: usize,
    iter: I,
    validator: fn(item: &I::Item) -> Result<bool, Error>,
    acceptable_except: Vec<error::Kind>,
    failed: bool
}

impl<I: Iterator> Iterator for FilterExcept<I> {
    type Item = Result<<I as Iterator>::Item, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }
        loop {
            let ret = self.iter.next();
            match ret {
                None => { return None; }
                Some(v) => {
                    let valid_ret = (self.validator)(&v);
                    match valid_ret {
                        Err(e) => {
                            if self.acceptable_except.contains(&e.kind()) {
                                continue;
                            } else {
                                self.failed = true;
                                match e.message() {
                                    None=> { return Some(Err(error::any_error(e.kind(), "validator failed".to_string()))); }
                                    Some(m) => { return Some(Err(error::any_error(e.kind(), "validator failed: ".to_string()+m))); }
                                }
                            }   
                        }
                        Ok(_) => {
                            return Some(Ok(v));
                        }
                    }
                }
            }
        }
    }

}


pub fn filter_except<I>(iterable: I, 
                        validator: fn(item: &I::Item) -> Result<bool, Error>,
                        acceptable_except: Vec<error::Kind>) -> FilterExcept<I::IntoIter> 
where
    I: IntoIterator,
{
                            
    FilterExcept {
        // cur: 0,
        iter: iterable.into_iter(),
        validator: validator,
        acceptable_except: acceptable_except,
        failed: false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fe = filter_except(iterable, 
            |x| { 
                let ret  = x.parse::<i32>();
                match ret {
                    Ok(_) => { return Ok(true); }
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }, 
            vec![error::Kind::ValueError]);

        match fe.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!("1", v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!("2", v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!("4", v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!("5", v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(_) => { assert!(false);},
            None => { assert!(true); }
        }
        match fe.next() {
            Some(_) => { assert!(false);},
            None => { assert!(true); }
        }
    }

    #[test]
    fn test2() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fe = filter_except(iterable, 
            |x| { 
                let ret  = x.parse::<i32>();
                match ret {
                    Ok(_) => { return Ok(true); }
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }, 
            vec![]);

        match fe.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!("1", v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!("2", v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(v) => {
                match v {
                    Ok(_) => { assert!(false); }
                    Err(_) => { assert!(true); }
                }
            },
            None => {}
        }
        match fe.next() {
            Some(_) => { assert!(false);},
            None => { assert!(true); }
        }
        match fe.next() {
            Some(_) => { assert!(false);},
            None => { assert!(true); }
        }
    }
}