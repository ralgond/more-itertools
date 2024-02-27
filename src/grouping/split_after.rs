use std::collections::LinkedList;
use crate::error::Error;

struct SplitAfterItem<T> {
    items: Vec<T>,
    splited: bool // 
}

pub struct SplitAfter<T> {
    output_item_list: LinkedList<SplitAfterItem<T>>,
    iter: Box<dyn Iterator<Item = T>>,
    pred: fn(&T) -> bool,
    split_cnt: usize,
    maxsplit: i128,
    iter_finished: bool
}


impl<T> Iterator for SplitAfter<T> {
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.maxsplit == 0i128 {
            if self.iter_finished {
                return None;
            }

            if self.output_item_list.len() == 0 {
                self.output_item_list.push_back(SplitAfterItem{
                    items: Vec::new(),
                    splited: false
                })
            }
            loop {
                let _next = self.iter.next();
                match _next {
                    None => { 
                        self.iter_finished = true;
                        return Some(Ok(self.output_item_list.pop_front().unwrap().items)); 
                    }
                    Some(v) => { self.output_item_list.back_mut().unwrap().items.push(v) }
                }
            }
        }

        loop {
            // consume all SpliteBeforeItem
            if self.output_item_list.len() == 0 && self.iter_finished {
                return None;
            }
            if self.output_item_list.len() > 0 {
                let front = self.output_item_list.front_mut().unwrap();
                if front.splited {
                    if front.items.len() == 0 {
                        self.output_item_list.pop_front();
                        continue;
                    } else {
                        let mut ret = Vec::new();
                        std::mem::swap(&mut front.items, &mut ret);
                        return Some(Ok(ret));
                    }
                }
            }

            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    if self.output_item_list.len() > 0 {
                        self.output_item_list.back_mut().unwrap().splited = true;
                    }
                },
                Some(v) => {
                    if self.maxsplit >= 0 {
                        if (self.split_cnt as i128) < self.maxsplit && (self.pred)(&v) {
                            self.split_cnt += 1;

                            if self.output_item_list.len() == 0 || self.output_item_list.back_mut().unwrap().splited {
                                self.output_item_list.push_back(SplitAfterItem {
                                    items: Vec::new(),
                                    splited: true
                                });
                                self.output_item_list.back_mut().unwrap().items.push(v);
                            } else {
                                self.output_item_list.back_mut().unwrap().items.push(v);
                                self.output_item_list.back_mut().unwrap().splited = true;
                            }
                        } else {
                            if self.output_item_list.len() == 0 {
                                self.output_item_list.push_back(SplitAfterItem {
                                    items: Vec::new(),
                                    splited: false
                                });
                            }
                            self.output_item_list.back_mut().unwrap().items.push(v);
                        }
                    } else {
                        if (self.pred)(&v) {
                            if self.output_item_list.len() == 0 || self.output_item_list.back_mut().unwrap().splited {
                                self.output_item_list.push_back(SplitAfterItem {
                                    items: Vec::new(),
                                    splited: true
                                });
                                self.output_item_list.back_mut().unwrap().items.push(v);
                            } else {
                                self.output_item_list.back_mut().unwrap().items.push(v);
                                self.output_item_list.back_mut().unwrap().splited = true;
                            }
                        } else {
                            if self.output_item_list.len() == 0 {
                                self.output_item_list.push_back(SplitAfterItem {
                                    items: Vec::new(),
                                    splited: false
                                });
                            }
                            self.output_item_list.back_mut().unwrap().items.push(v);
                        }
                    }
                }
            }
        }
    }
}


pub fn splite_after<T>(iter: Box<dyn Iterator<Item = T>>, 
    pred: fn(&T) -> bool,
    maxsplit: i128
) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where 
T: 'static
{
    Box::new(SplitAfter {
        output_item_list: LinkedList::new(),
        iter,
        pred,
        split_cnt: 0,
        maxsplit,
        iter_finished: false
    })
}


#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_after(iter_from_vec(v), |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![1,2,3])), r.next());
        assert_eq!(Some(Ok(vec![4,5,6])), r.next());
        assert_eq!(Some(Ok(vec![7,8,9])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_after(iter_from_vec(v), |x|{x%3==0}, 2);
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![1,2,3])), r.next());
        assert_eq!(Some(Ok(vec![4,5,6,7,8,9])), r.next());

        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_after(iter_from_vec(v), |x|{x%3==0}, 0);
        assert_eq!(Some(Ok(vec![0,1,2,3,4,5,6,7,8,9])), r.next());
        assert_eq!(None, r.next());
    }
}