use crate::error::Error;

#[derive(Debug,Clone)]
pub struct Iterate<T> {
    func: fn(&T) -> Result<T,Error>,
    start: T,
    error: Option<Error>,
    iter_finished: bool
}

impl<T> Iterator for Iterate<T> 
where 
T: Clone
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }
        
        if let Some(v_err) = &self.error {
            self.iter_finished = true;
            return Some(Err(v_err.clone()));
        }

        let ret = self.start.clone();
        let ret_func = (self.func)(&self.start);
        match ret_func {
            Ok(ok_ret_func) => {
                self.start = ok_ret_func;
            },
            Err(err_ret_func) => {
                self.error = Some(err_ret_func);
            }
        }
        
        return Some(Ok(ret));
    }
}


pub fn iterate<T>(func: fn(&T) -> Result<T,Error>, start: T) -> Box<dyn Iterator<Item = Result<T,Error>>>
where
T: Clone + 'static
{
    Box::new(Iterate {
        func: func,
        start: start,
        error: None,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {
    use crate::{error, itertools::islice::islice, utils::extract_value_from_result_vec};

    use super::*;

    #[test]
    fn test1() {
        let mut it = iterate(|x| { Ok(x * 2) }, 1);
        assert_eq!(Some(Ok(1)), it.next());
        assert_eq!(Some(Ok(2)), it.next());
        assert_eq!(Some(Ok(4)), it.next());
        assert_eq!(Some(Ok(8)), it.next());
        assert_eq!(Some(Ok(16)), it.next());
    }
 
    #[test]
    fn test2() {
        let i = islice(iterate(|x| { Ok(x * 2) }, 1), 0, 10, 1);
        let j = extract_value_from_result_vec(i.collect::<Vec<_>>());
        // println!("{:?}", j.0);
        assert_eq!(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512], j.0);
    }

    #[test]
    fn test3() {
        let i = islice(iterate(|_| { Err(error::overflow_error("[test]".to_string())) }, 1), 0, 10, 1);
        let j = extract_value_from_result_vec(i.collect::<Vec<_>>());
        // println!("{:?}", j.0);
        assert_eq!(vec![1], j.0);
        assert_eq!(error::Kind::OverflowError, j.1.unwrap().kind());
    }
}