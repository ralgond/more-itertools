use crate::error::Error;
use crate::error;

pub struct MapExcept<I, T> {
    // cur: usize,
    iter: Box<dyn Iterator<Item = I>>,
    func: fn(item: &I) -> Result<T, Error>,
    acceptable_except: Vec<error::Kind>,
    failed: bool
}

impl<I, T> Iterator for MapExcept<I, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }

        loop {
            let ret = self.iter.next();
            match ret {
                None => { return None; }
                Some(v) => {
                    let res_func_ret = (self.func)(&v);
                    match res_func_ret {
                        Ok(func_ret) => {
                            return Some(Ok(func_ret));
                        },
                        Err(e) => {
                            if self.acceptable_except.contains(&e.kind()) {
                                continue;
                            } else {
                                self.failed = true;
                                match e.message() {
                                    None => { return Some(Err(error::any_error(e.kind(), "func failed".to_string()))); }
                                    Some(m) => { return Some(Err(error::any_error(e.kind(), "func failed: ".to_string()+m))); }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}

pub fn map_except<I, T>(iter: Box<dyn Iterator<Item = I>>, 
    func: fn(item: &I) -> Result<T, Error>,
    acceptable_except: Vec<error::Kind>) -> Box<dyn Iterator<Item = Result<T, Error>>> 
where
I: 'static,
T: 'static
{  
    Box::new(MapExcept {
        // cur: 0,
        iter,
        func,
        acceptable_except,
        failed: false
    })
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fm = map_except(iter_from_vec(iterable),
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
        let mut fm = map_except(iter_from_vec(iterable),
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
}