use std::collections::LinkedList;
use crate::error::Error;

struct SplitBeforeItem<T> {
    items: Vec<T>,
    splited: bool // 
}

// impl<T> SplitBeforeItem<T> {
//     // pub fn pop_front(&mut self) -> Option<T> {
//     //     self.items.pop_front()
//     // }
//     // pub fn push_back(&mut self, item: T) {
//     //     self.items.push_back(item)
//     // }
//     pub fn len(&self) -> usize {
//         return self.items.len();
//     }
// }

pub struct SplitBefore<T> {
    output_item_list: LinkedList<SplitBeforeItem<T>>,
    iter: Box<dyn Iterator<Item = T>>,
    pred: fn(&T) -> bool,
    split_cnt: usize,
    maxsplit: i128,
    iter_finished: bool
}

impl<T> Iterator for SplitBefore<T> {
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // consume all SplitBeforeItem
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
                        if (self.split_cnt as i128) <= self.maxsplit && (self.pred)(&v) {
                            self.split_cnt += 1;

                            if self.output_item_list.len() > 0 {
                                self.output_item_list.back_mut().unwrap().splited = true;
                            }

                            self.output_item_list.push_back(SplitBeforeItem{
                                items: Vec::new(),
                                splited: false
                            });
                            self.output_item_list.back_mut().unwrap().items.push(v);
                        } else {
                            if self.output_item_list.len() == 0 || self.output_item_list.back().unwrap().splited {
                                self.output_item_list.push_back(SplitBeforeItem{
                                    items: Vec::new(),
                                    splited: false
                                });
                            }
                            self.output_item_list.back_mut().unwrap().items.push(v);
                        }
                    } else {
                        if (self.pred)(&v) {
                            if self.output_item_list.len() > 0 {
                                self.output_item_list.back_mut().unwrap().splited = true;
                            }

                            self.output_item_list.push_back(SplitBeforeItem{
                                items: Vec::new(),
                                splited: false
                            });
                            self.output_item_list.back_mut().unwrap().items.push(v);

                        } else {
                            if self.output_item_list.len() == 0 || self.output_item_list.back().unwrap().splited {
                                self.output_item_list.push_back(SplitBeforeItem{
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

pub fn split_before<T>(iter: Box<dyn Iterator<Item = T>>, 
    pred: fn(&T) -> bool,
    maxsplit: i128
) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where 
T: 'static
{
    Box::new(SplitBefore {
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
        let mut r = split_before(iter_from_vec(v), |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![0,1,2])), r.next());
        assert_eq!(Some(Ok(vec![3,4,5])), r.next());
        assert_eq!(Some(Ok(vec![6,7,8])), r.next());
        assert_eq!(Some(Ok(vec![9])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = split_before(iter_from_vec(v), |x|{x%3==0}, 2);
        assert_eq!(Some(Ok(vec![0,1,2])), r.next());
        assert_eq!(Some(Ok(vec![3,4,5])), r.next());
        assert_eq!(Some(Ok(vec![6,7,8,9])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![0,0,0];
        let mut r = split_before(iter_from_vec(v), |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![1,1,1];
        let mut r = split_before(iter_from_vec(v), |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![1,1,1])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![1,1,1];
        let mut r = split_before(iter_from_vec(v), |x|{x%3==0}, 2);
        assert_eq!(Some(Ok(vec![1,1,1])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());
    }
}