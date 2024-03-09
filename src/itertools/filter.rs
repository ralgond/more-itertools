use crate::error::Error;
use crate::error;

struct Filter<T> {
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    iter_finished: bool,
    pred: fn(&T) -> Result<bool, Error>
}

impl<T> Iterator for Filter<T> 
where T: 'static
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        loop {
            let _next = self.iter.next();
            if let Some(v_next) = _next {
                match v_next {
                    Ok(ok_v_next) => {
                        match (self.pred)(&ok_v_next) {
                            Ok(ok_pred_ret) => {
                                if ok_pred_ret {
                                    return Some(Ok(ok_v_next));
                                } else {
                                    continue;
                                }
                            },
                            Err(err_pred_ret) => {
                                self.iter_finished = true;
                                return Some(Err(error::any_error(err_pred_ret.kind(), "[filter] ".to_string()+err_pred_ret.message().unwrap())));
                            }
                        }
                    },
                    Err(err_v_next) => { // upstream error
                        self.iter_finished = true;
                        return Some(Err(err_v_next));
                    }
                }
            } else {
                self.iter_finished = true;
                return None;
            }
        }
    }
}

pub fn filter<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, pred: fn(&T) -> Result<bool,Error>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where T: 'static
{
    return Box::new(Filter{
        iter,
        iter_finished: false,
        pred
    });
}

#[cfg(test)]
mod tests {

    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator}};

    use super::filter;


    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9];
        let ret = filter(generate_okok_iterator(v), |x| { Ok(x % 2 == 0) }).collect::<Vec<_>>();
        assert_eq!(vec![2,4,6,8], extract_value_from_result_vec(ret).0);

        let v = vec![1,2];
        let mut ret = filter(generate_okok_iterator(v), 
                                                                |x| { 
                                                                    if *x < 2 {
                                                                        return Ok(true);
                                                                    } else {
                                                                        return Err(error::overflow_error("[test]".to_string()));
                                                                    }
                                                                 });
        assert_eq!(1, ret.next().unwrap().ok().unwrap());
        assert_eq!(error::Kind::OverflowError, ret.next().unwrap().err().unwrap().kind());
        assert_eq!(None, ret.next());
    }
}