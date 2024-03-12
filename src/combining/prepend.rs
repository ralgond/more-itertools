use crate::error::Error;

struct Prepend<T> {
    value: T,
    emitted_value: bool,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    iter_finished: bool
}

impl<T> Iterator for Prepend<T> 
where T: Clone + 'static
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if !self.emitted_value {
            self.emitted_value = true;
            return Some(Ok(self.value.clone()));
        }

        let _next = self.iter.next();
        if let Some(v) = _next {
            match v {
                Ok(ok_v) => {
                    return Some(Ok(ok_v));
                },
                Err(err_v) => { // upstream error
                    self.iter_finished = true;
                    return Some(Err(err_v));
                }
            }
        } else {
            self.iter_finished = true;
            return None;
        }
    }
}

pub fn prepend<T>(value: T, iter: Box<dyn Iterator<Item = Result<T,Error>>>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where
T: Clone + 'static
{
    Box::new(Prepend {
        value,
        emitted_value: false,
        iter,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {
    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let mut iter = prepend(0, generate_okok_iterator(vec![1,2,3]));
        assert_eq!(Ok(0), iter.next().unwrap());
        assert_eq!(Ok(1), iter.next().unwrap());
        assert_eq!(Ok(2), iter.next().unwrap());
        assert_eq!(Ok(3), iter.next().unwrap());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());


        let iter = prepend(0, generate_okokerr_iterator(vec![1,2,3], error::overflow_error("[test]".to_string())));
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        assert_eq!(vec![0,1,2,3], ret.0);
        assert_eq!(error::Kind::OverflowError, ret.1.unwrap().kind());
    }
}