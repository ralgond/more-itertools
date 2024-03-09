use crate::error::Error;

pub struct RepeatLast<T> 
where
T: Clone
{
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    default_item: Option<T>,
    last_item: Option<Result<T,Error>>,
    iter_finished: bool,
}


impl<T> Iterator for RepeatLast<T>
where
T: Clone
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                match &self.last_item {
                    None => {
                        return Some(Ok(self.default_item.as_mut().unwrap().clone()));
                    },
                    Some(v) => {
                        match v {
                            Ok(ok_v) => {
                                return Some(Ok(ok_v.clone()));
                            },
                            Err(_) => {
                                return None; // when upstream error, stop this iterator
                            }
                        }
                        
                    }
                }
            }

            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                },
                Some(v) => {
                    match v {
                        Ok(ok_v) => {
                            self.last_item = Some(Ok(ok_v.clone()));
                            return Some(Ok(ok_v));
                        },
                        Err(err_v) => {
                            // upstream error
                            self.iter_finished = true;
                            self.last_item = Some(Err(err_v.clone()));
                            return Some(Err(err_v));
                        }
                    }
                }
            }
        }
    }
}

pub fn repeat_last<T>(iter: Box<dyn Iterator<Item=Result<T,Error>>>, default_item: T) -> Box<dyn Iterator<Item=Result<T,Error>>>
where
T: Clone + 'static
{
    Box::new(RepeatLast {
        iter,
        default_item: Some(default_item),
        last_item: None,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {

    use crate::{error, itertools::islice::islice, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {

        let rl = repeat_last(generate_okok_iterator(vec![1,2,3]), 0);
        assert_eq!((vec![1, 2, 3, 3, 3], None), extract_value_from_result_vec(islice(rl, 0, 5, 1).collect::<Vec<_>>()));

        let rl = repeat_last(generate_okok_iterator(Vec::<i32>::new()), 42);
        assert_eq!((vec![42,42,42,42,42], None), extract_value_from_result_vec(islice(rl, 0, 5, 1).collect::<Vec<_>>()));

        let rl = repeat_last(generate_okokerr_iterator(vec![1,2,3], error::overflow_error("[test]".to_string())), 0);
        let mut ret = islice(rl, 0, 5, 1);
        assert_eq!(Ok(1), ret.next().unwrap());
        assert_eq!(Ok(2), ret.next().unwrap());
        assert_eq!(Ok(3), ret.next().unwrap());
        assert_eq!(error::Kind::OverflowError, ret.next().unwrap().err().unwrap().kind());
        assert_eq!(None, ret.next());
    }
}