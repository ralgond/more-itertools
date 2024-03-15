use std::collections::LinkedList;

use crate::error::{self, Error};

struct SplitAtOutputItem<T> {
    is_sep: bool,
    items: Vec<T>,
    error: Option<Error>,
    finished: bool,
}


impl<T> SplitAtOutputItem<T> {
    pub fn new(is_sep: bool, items: Vec<T>, error: Option<Error>, finished: bool,) -> Self {
        return SplitAtOutputItem {
            is_sep,
            items,
            error,
            finished
        };
    }
}

pub struct SplitAt<T> {
    ret_buf: LinkedList<SplitAtOutputItem<T>>,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    pred: fn(&T) -> Result<bool,Error>,
    maxsplit: i128,
    keep_separator: bool,
    splited: i128,
    iter_finished: bool
}

impl<T> SplitAt<T> {
    pub fn set_last_output_item_finished(&mut self) {
        if self.ret_buf.len() > 0 {
            self.ret_buf.back_mut().unwrap().finished = true;
        }
    }
}

impl<T> Iterator for SplitAt<T> 
{
    type Item = Result<Vec<T>,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        
        loop {
            while self.ret_buf.len() > 0 {
                if self.ret_buf.front().unwrap().finished {
                    let ret = self.ret_buf.pop_front();
                    if ret.as_ref().unwrap().error.is_none() {
                        if !self.keep_separator && ret.as_ref().unwrap().is_sep {
                            continue;
                        }
                        return Some(Ok(ret.unwrap().items)); 
                    } else {
                        return Some(Err(ret.unwrap().error.unwrap()));
                    }
                } else {
                    break; // jump to eating logic
                }
            }

            // eating logic
            if self.iter_finished {
                return None;
            }

            if let Some(v) = self.iter.next() {
                match v {
                    Ok(ok_v) => {
                        let pred_ret = (self.pred)(&ok_v);
                        match pred_ret {
                            Ok(ok_pred_ret) => {
                                if ok_pred_ret && (self.maxsplit < 0 || self.splited < self.maxsplit) { // meet a seperator
                                    self.splited += 1;
                                    self.set_last_output_item_finished();
                                    self.ret_buf.push_back(SplitAtOutputItem::new(true, vec![ok_v], None, true));
                                    self.ret_buf.push_back(SplitAtOutputItem::new(false, Vec::new(), None, false));
                                } else {
                                    self.ret_buf.back_mut().unwrap().items.push(ok_v);
                                }
                            },
                            Err(err_pred_ret) => {
                                self.set_last_output_item_finished();
                                self.ret_buf.push_back(SplitAtOutputItem::new(false, Vec::new(), Some(error::any_error(err_pred_ret.kind(), "[split_at] ".to_string()+ err_pred_ret.message().unwrap())), true));
                                self.iter_finished = true;
                            }
                        }
                    },
                    Err(err_v) => {
                        self.set_last_output_item_finished();
                        self.ret_buf.push_back(SplitAtOutputItem::new(false, Vec::new(), Some(err_v), true));
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

pub fn split_at<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, pred: fn(&T)->Result<bool,Error>, maxsplit: i128, keep_separator: bool) -> Box<dyn Iterator<Item=Result<Vec<T>,Error>>>
where
T: 'static
{
    let mut ret = SplitAt {
        ret_buf: LinkedList::new(),
        iter,
        pred,
        maxsplit,
        keep_separator,
        splited: 0,
        iter_finished: false
    };

    ret.ret_buf.push_back(SplitAtOutputItem {
        is_sep: false,
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
        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==3)}, -1, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        assert_eq!(vec![vec![1,2],vec![3],vec![4,5]], ret.0);

        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==1 || *x==5)}, -1, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        assert_eq!(vec![vec![],vec![1],vec![2,3,4],vec![5],vec![]], ret.0);

        let v = vec![3,3,3];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==3)}, -1, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        assert_eq!(vec![vec![],vec![3],vec![],vec![3],vec![],vec![3],vec![]], ret.0);
        // println!("{:?}", ret);
    }

    #[test]
    fn test1_maxsplit() {

        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==1 || *x==5)}, 1, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![],vec![1],vec![2,3,4,5]], ret.0);

        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==1 || *x==5)}, 0, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1,2,3,4,5]], ret.0);
    }

    #[test]
    fn test1_keep_sep() {
        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==3)}, -1, false);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        //assert_eq!(vec![vec![1,2],vec![3],vec![4,5]], ret.0);
        println!("{:?}", ret);

        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), |x|{Ok(*x==1 || *x==5)}, -1, false);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        //assert_eq!(vec![vec![],vec![1],vec![2,3,4],vec![5],vec![]], ret.0);
        println!("{:?}", ret);
    }

    #[test]
    fn test1_error() {
        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), |x|{Ok(*x==3)}, -1, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        assert_eq!(vec![vec![1,2],vec![3],vec![4,5]], ret.0);
        assert_eq!(error::Kind::OverflowError, ret.1.as_ref().unwrap().kind());

        let v = vec![1,2,3,4,5];
        let iter = split_at(generate_okok_iterator(v), 
                                                                    |x|{  if *x == 4 { Err(error::overflow_error("[test]".to_string()))} else {Ok(*x==3)} }, 
                                                                    -1, true);
        let ret = extract_value_from_result_vec(iter.collect::<Vec<_>>());
        assert_eq!(vec![vec![1,2],vec![3],vec![]], ret.0);
        assert_eq!(error::Kind::OverflowError, ret.1.as_ref().unwrap().kind());
    }

    #[test]
    fn test2() {
    }
}