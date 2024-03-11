use crate::error::Error;

struct RepeatAppendDefault<T> {
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    iter_finished: bool,
    default_value: Option<T>,
    error: Option<Error>
}

impl<T> Iterator for RepeatAppendDefault<T> 
where T: Clone
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                if let Some(_) = &self.error {
                    return None;
                } else {
                    if let Some(v_default) = &self.default_value {
                        return Some(Ok(v_default.clone()));
                    } else {
                        return None;
                    }
                }
            } else {
                if let Some(v) = self.iter.next() {
                    match v {
                        Ok(ok_v) => {
                            return Some(Ok(ok_v));
                        },
                        Err(err_v) => { // upstream error
                            self.error = Some(err_v);
                            self.iter_finished = true;
                            return Some(Err(self.error.as_ref().unwrap().clone()));
                        }
                    }
                } else {
                    self.iter_finished = true;
                    continue;
                }
            }
        }
    }
}

pub fn repeat_append_default<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, default_value: Option<T>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where T: Clone + 'static
{
    return Box::new(RepeatAppendDefault{
        iter,
        iter_finished: false,
        default_value,
        error: None
    });
}

#[cfg(test)]
mod tests {
    use crate::{error, itertools::islice::islice, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let rab = repeat_append_default(generate_okok_iterator(vec![1,2,3]), Some(4));
        let ret = islice(rab, 0, 5, 1).collect::<Vec<_>>();
        assert_eq!(vec![1,2,3,4,4], extract_value_from_result_vec(ret).0);

        let rab = repeat_append_default(generate_okok_iterator(vec![1,2,3]), None);
        let ret = islice(rab, 0, 5, 1).collect::<Vec<_>>();
        assert_eq!(vec![1,2,3], extract_value_from_result_vec(ret).0);

        let rab = repeat_append_default(generate_okokerr_iterator(vec![1,2,3], error::overflow_error("[test]".to_string())), Some(4));
        let ret = islice(rab, 0, 5, 1).collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![1,2,3], ret2.0);
        assert_eq!(error::Kind::OverflowError, ret2.1.unwrap().kind());
    }
}

