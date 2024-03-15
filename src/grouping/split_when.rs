use std::collections::LinkedList;
use crate::error::{self, Error};

struct SplitWhenOutputItem<T> {
    items: Vec<T>,
    error: Option<Error>,
    finished: bool
}

impl<T> SplitWhenOutputItem<T> {
    pub fn new(items: Vec<T>, error: Option<Error>, finished: bool) -> Self {
        return SplitWhenOutputItem {
            items,
            error,
            finished
        };
    }
}

pub struct SplitWhen<T> {
    ret_buf: LinkedList<SplitWhenOutputItem<T>>,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    pred: fn(&T, &T) -> Result<bool, Error>,
    maxsplit: i128,
    splited: i128,
    iter_finished: bool
}

impl<T> SplitWhen<T> {
    pub fn set_last_output_item_finished(&mut self) {
        if self.ret_buf.len() > 0 {
            self.ret_buf.back_mut().unwrap().finished = true;
        }
    }
}

impl<T> Iterator for SplitWhen<T>
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // flush the cached data
            while self.ret_buf.len() > 0 {
                if self.ret_buf.front().unwrap().finished {
                    let ret = self.ret_buf.pop_front();
                    if ret.as_ref().unwrap().error.is_none() {
                        return Some(Ok(ret.unwrap().items)); 
                    } else {
                        return Some(Err(ret.unwrap().error.unwrap()));
                    }
                } else {
                    break; // jump to consume logic
                }
            }

            // consume the iterator
            if self.iter_finished {
                return None;
            }

            if let Some(v) = self.iter.next() {
                match v {
                    Ok(ok_v) => {
                        if self.ret_buf.back_mut().unwrap().items.len() < 1 {
                            self.ret_buf.back_mut().unwrap().items.push(ok_v);
                            continue;
                        }

                        // main logic
                        let pred_ret = (self.pred)(self.ret_buf.back_mut().unwrap().items.last().unwrap(), &ok_v);
                        match pred_ret {
                            Ok(ok_pred_ret) => {
                                if ok_pred_ret && (self.maxsplit < 0 || self.splited < self.maxsplit) {
                                    self.splited += 1;
                                    self.set_last_output_item_finished();
                                    self.ret_buf.push_back(SplitWhenOutputItem::new(vec![ok_v], None, false));
                                } else {
                                    self.ret_buf.back_mut().unwrap().items.push(ok_v);
                                }
                            },
                            Err(err_pred_ret) => {
                                self.set_last_output_item_finished();
                                self.ret_buf.push_back(SplitWhenOutputItem::new(Vec::new(), Some(error::any_error(err_pred_ret.kind(), "[split_when] ".to_string()+ err_pred_ret.message().unwrap())), true));
                                self.iter_finished = true;
                            }
                        }
                    },
                    Err(err_v) => { // upstream error
                        self.set_last_output_item_finished();
                        self.ret_buf.push_back(SplitWhenOutputItem::new(Vec::new(), Some(err_v), true));
                        self.iter_finished = true;
                    }
                }
            } else {
                self.set_last_output_item_finished();
                self.iter_finished = true;
                continue;
            }
        }
    }
}

pub fn split_when<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, pred: fn(&T, &T) -> Result<bool, Error>, maxsplit: i128) -> Box<dyn Iterator<Item = Result<Vec<T>,Error>>>
where
T: 'static
{
    let mut ret = SplitWhen {
        ret_buf: LinkedList::new(),
        iter,
        pred,
        maxsplit,
        splited: 0,
        iter_finished: false
    };

    ret.ret_buf.push_back(SplitWhenOutputItem {
        items: Vec::new(),
        error: None,
        finished: false
    });

    return Box::new(ret);
}

#[cfg(test)]
mod tests {
    use crate::utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator};

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(generate_okok_iterator(v), |x, y| { Ok(x < y) }, -1);
        let ret = extract_value_from_result_vec(sw.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1], vec![2], vec![3, 3, 2], vec![5, 2], vec![4, 2]], ret.0);


        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(generate_okok_iterator(v), |x, y| { Ok(x < y) }, 2);
        let ret = extract_value_from_result_vec(sw.collect::<Vec<_>>());
        assert_eq!(vec![vec![1], vec![2], vec![3, 3, 2, 5, 2, 4, 2]], ret.0);

        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(generate_okok_iterator(v), |x, y| { Ok(x < y) }, 10);
        let ret = extract_value_from_result_vec(sw.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1], vec![2], vec![3, 3, 2], vec![5, 2], vec![4, 2]], ret.0);

        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(generate_okok_iterator(v), |x, y| { Ok(x < y) }, 0);
        let ret = extract_value_from_result_vec(sw.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1, 2, 3, 3, 2, 5, 2, 4, 2]], ret.0);
    }

    #[test]
    fn test1_error() {
        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), |x, y| { Ok(x < y) }, -1);
        let ret = extract_value_from_result_vec(sw.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1], vec![2], vec![3, 3, 2], vec![5, 2], vec![4, 2]], ret.0);
        assert_eq!(error::Kind::OverflowError, ret.1.unwrap().kind());


        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), 
                                                        |x, y| { if *y == 5 { Err(error::overflow_error("[test]".to_string())) } else { Ok(x < y)} }, -1);
        let ret = extract_value_from_result_vec(sw.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1], vec![2], vec![3, 3, 2]], ret.0);
        assert_eq!(error::Kind::OverflowError, ret.1.unwrap().kind());
    }
}