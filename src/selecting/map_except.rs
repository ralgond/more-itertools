use crate::error::Error;
use crate::error;

pub struct MapExcept<I, T> {
    // cur: usize,
    iter: Box<dyn Iterator<Item = Result<I,Error>>>,
    func: fn(item: &I) -> Result<T, Error>,
    acceptable_except: Vec<error::Kind>,
    error: Option<Error>,
    iter_finished: bool
}

impl<I, T> Iterator for MapExcept<I, T> {
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
                    return Some(Err(v.as_ref().err().unwrap().clone()));
                }

                let val_result = (self.func)(v.as_ref().ok().unwrap());
                match val_result {
                    Ok(ok_val_result) => {
                        return Some(Ok(ok_val_result));
                    },
                    Err(err_val_result) => {
                        if self.acceptable_except.contains(&err_val_result.kind()) {
                            continue;
                        } else {
                            self.iter_finished = true;
                            return Some(Err(error::any_error(err_val_result.kind(), 
                                        "[map_except] ".to_string() + err_val_result.message().unwrap())));
                        }
                    }
                }
            } else {
                self.iter_finished = true;
                return None
            }

            
        }

        // loop {
        //     let ret = self.iter.next();
        //     match ret {
        //         None => { return None; }
        //         Some(v) => {
        //             let res_func_ret = (self.func)(&v);
        //             match res_func_ret {
        //                 Ok(func_ret) => {
        //                     return Some(Ok(func_ret));
        //                 },
        //                 Err(e) => {
        //                     if self.acceptable_except.contains(&e.kind()) {
        //                         continue;
        //                     } else {
        //                         self.failed = true;
        //                         match e.message() {
        //                             None => { return Some(Err(error::any_error(e.kind(), "func failed".to_string()))); }
        //                             Some(m) => { return Some(Err(error::any_error(e.kind(), "func failed: ".to_string()+m))); }
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

}

pub fn map_except<I, T>(iter: Box<dyn Iterator<Item = Result<I,Error>>>, 
    func: fn(item: &I) -> Result<T, Error>,
    acceptable_except: Vec<error::Kind>) -> Box<dyn Iterator<Item = Result<T, Error>>> 
where
I: 'static,
T: 'static
{  
    Box::new(MapExcept {
        iter,
        func,
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
        let mut fm = map_except(generate_okok_iterator(iterable),
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok(v); },
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            },
            vec![error::Kind::ValueError]
        );

        assert_eq!(1, fm.next().unwrap().ok().unwrap());
        assert_eq!(2, fm.next().unwrap().ok().unwrap());
        assert_eq!(4, fm.next().unwrap().ok().unwrap());
        assert_eq!(5, fm.next().unwrap().ok().unwrap());
        assert_eq!(None, fm.next());
        assert_eq!(None, fm.next());
    }


    #[test]
    fn test2() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fm = map_except(generate_okok_iterator(iterable),
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok(v); },
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            },
            vec![]
        );

        assert_eq!(1, fm.next().unwrap().ok().unwrap());
        assert_eq!(2, fm.next().unwrap().ok().unwrap());
        assert_eq!(error::Kind::ValueError, fm.next().unwrap().err().unwrap().kind());
        assert_eq!(None, fm.next());
        assert_eq!(None, fm.next());
    }

    #[test]
    fn test3() {
        let iterable = vec!["1", "2"];
        let mut fm = map_except(generate_okokerr_iterator(iterable, error::overflow_error("[test]".to_string())),
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok(v); },
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            },
            vec![]
        );

        assert_eq!(1, fm.next().unwrap().ok().unwrap());
        assert_eq!(2, fm.next().unwrap().ok().unwrap());
        assert_eq!(error::Kind::OverflowError, fm.next().unwrap().err().unwrap().kind());
    }
}