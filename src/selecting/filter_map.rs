use crate::error::Error;
use crate::error;

pub struct FilterMap<I, T> {
    // cur: usize,
    iter: Box<dyn Iterator<Item=Result<I,Error>>>,
    func: fn(item: &I) -> Result<(T,bool), Error>, // Result<(T,bool), bool means whether emmit current item.
    iter_finished: bool
}

impl<I, T> Iterator for FilterMap<I, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            let ret = self.iter.next();
            if let Some(v_ret) = ret {
                if let Err(err_v_ret) = v_ret {
                    self.iter_finished = true;
                    return Some(Err(err_v_ret));
                }

                let func_ret = (self.func)(&v_ret.ok().unwrap());
                match func_ret {
                    Ok(ok_func_ret) => {
                        if ok_func_ret.1 {
                            return Some(Ok(ok_func_ret.0));
                        } else {
                            continue;
                        }
                    },
                    Err(err_func_ret) => {
                        self.iter_finished = true;
                        return Some(Err(error::any_error(err_func_ret.kind(), "[filter_map] ".to_string()+err_func_ret.message().unwrap())));
                    }
                }
            } else {
                self.iter_finished = true;
                return None;
            }
        }
    }
}

pub fn filter_map<I, T>(iter: Box<dyn Iterator<Item = Result<I,Error>>>, 
    func: fn(item: &I) -> Result<(T,bool), Error>) -> Box::<dyn Iterator<Item = Result<T, Error>>>
where
I: 'static,
T: 'static
{  
    Box::new(FilterMap {
        iter,
        func: func,
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
        let mut fm = filter_map(generate_okok_iterator(iterable),
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok((v, true)); },
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }
        );

        assert_eq!(fm.next().unwrap().ok().unwrap(), 1);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 2);
        assert_eq!(error::Kind::ValueError, fm.next().unwrap().err().unwrap().kind());

        assert_eq!(None, fm.next());
        assert_eq!(None, fm.next());
    }

    #[test]
    fn test2() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fm = filter_map(generate_okok_iterator(iterable),
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok((v, true)); },
                    Err(_) => { return Ok((0,false)); }
                }
            }
        );

        assert_eq!(fm.next().unwrap().ok().unwrap(), 1);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 2);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 4);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 5);

        assert_eq!(None, fm.next());
        assert_eq!(None, fm.next());
    }

    #[test]
    fn test3() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fm = filter_map(
            generate_okokerr_iterator(iterable, error::overflow_error("for test".to_string())),
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok((v, true)); },
                    Err(_) => { return Ok((0,false)); }
                }
            }
        );

        assert_eq!(fm.next().unwrap().ok().unwrap(), 1);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 2);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 4);
        assert_eq!(fm.next().unwrap().ok().unwrap(), 5);
        assert_eq!(error::Kind::OverflowError, fm.next().unwrap().err().unwrap().kind());

        assert_eq!(None, fm.next());
        assert_eq!(None, fm.next());
    }
}