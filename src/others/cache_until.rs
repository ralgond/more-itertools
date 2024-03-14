use std::{collections::VecDeque, fmt::Debug};
use crate::{error, error::Error, utils::vecdeque_2_vec};

struct CacheUntil<T> {
    buf: VecDeque<T>,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    pred: fn(&T) -> Result<bool, Error>,
    pred_append_tail: bool,
    iter_finished: bool
}

impl<T> Iterator for CacheUntil<T> 
where T: Clone + Debug
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        loop {
            if let Some(_next) = self.iter.next() {
                match _next {
                    Ok(ok_next) => {
                        let pred_ret = (self.pred)(&ok_next);
                        
                        match pred_ret {
                            Ok(ok_pred_ret) => {
                                if ok_pred_ret {
                                    self.buf.push_back(ok_next);
                                    let ret;
                                    if self.pred_append_tail {
                                        ret = vecdeque_2_vec(&mut self.buf);
                                    } else {
                                        let pop_tail = self.buf.pop_back();
                                        ret = vecdeque_2_vec(&mut self.buf);
                                        self.buf.push_back(pop_tail.unwrap());
                                    }
                                    return Some(Ok(ret));
                                } else {
                                    self.buf.push_back(ok_next);
                                }
                            },
                            Err(err_pred_ret) => {
                                self.iter_finished = true;
                                return Some(Err(error::any_error(err_pred_ret.kind(), "[cache_util] ".to_string()+err_pred_ret.message().unwrap())));
                            }
                        }
                    },
                    Err(err_next) => { // upstream error
                        self.iter_finished = true;
                        return Some(Err(err_next));
                    }
                }
            } else {
                let ret = vecdeque_2_vec(&mut self.buf);
                self.iter_finished = true;
                return Some(Ok(ret));
            }
        }

    }
}

pub fn cache_util<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, pred: fn(&T) -> Result<bool, Error>, pred_append_tail: bool) -> Box<dyn Iterator<Item = Result<Vec<T>,Error>>> 
where T: Clone + Debug + 'static
{
    return Box::new(CacheUntil {
        buf: VecDeque::new(),
        iter,
        pred,
        pred_append_tail,
        iter_finished: false
    });
}


#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let mut it = cache_util(generate_okok_iterator(v), |x| {Ok(*x==1||*x==2||*x==5)}, true);
        assert_eq!(Some(Ok(vec![1])), it.next());
        assert_eq!(Some(Ok(vec![2])), it.next());
        assert_eq!(Some(Ok(vec![3, 4, 5])), it.next());
        assert_eq!(Some(Ok(Vec::<i32>::new())), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3,4,5];
        let mut it = cache_util(generate_okok_iterator(v), |x| {Ok(*x==1||*x==2||*x==5)}, false);
        assert_eq!(Some(Ok(Vec::<i32>::new())), it.next());
        assert_eq!(Some(Ok(vec![1])), it.next());
        assert_eq!(Some(Ok(vec![2,3,4])), it.next());
        assert_eq!(Some(Ok(vec![5])), it.next());
        assert_eq!(None, it.next());
    }
}