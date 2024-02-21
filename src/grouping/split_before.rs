use std::collections::LinkedList;
use crate::error::Error;

struct SpliteBeforeItem<T> {
    items: Vec<T>,
    splited: bool // 
}

// impl<T> SpliteBeforeItem<T> {
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

pub struct SpliteBefore<I: Iterator> {
    output_item_list: LinkedList<SpliteBeforeItem<I::Item>>,
    iter: I,
    pred: fn(&I::Item) -> bool,
    split_cnt: usize,
    maxsplit: i128,
    iter_finished: bool
}

impl<I: Iterator> Iterator for SpliteBefore<I> {
    type Item = Result<Vec<<I as Iterator>::Item>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
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
                        if (self.split_cnt as i128) <= self.maxsplit && (self.pred)(&v) {
                            self.split_cnt += 1;

                            if self.output_item_list.len() > 0 {
                                self.output_item_list.back_mut().unwrap().splited = true;
                            }

                            self.output_item_list.push_back(SpliteBeforeItem{
                                items: Vec::new(),
                                splited: false
                            });
                            self.output_item_list.back_mut().unwrap().items.push(v);
                        } else {
                            if self.output_item_list.len() == 0 || self.output_item_list.back().unwrap().splited {
                                self.output_item_list.push_back(SpliteBeforeItem{
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

                            self.output_item_list.push_back(SpliteBeforeItem{
                                items: Vec::new(),
                                splited: false
                            });
                            self.output_item_list.back_mut().unwrap().items.push(v);

                        } else {
                            if self.output_item_list.len() == 0 || self.output_item_list.back().unwrap().splited {
                                self.output_item_list.push_back(SpliteBeforeItem{
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

pub fn splite_before<I>(into_iter_able: I, 
    pred: fn(&I::Item) -> bool,
    maxsplit: i128
) -> SpliteBefore<I::IntoIter>
where 
I: IntoIterator
{
    SpliteBefore {
        output_item_list: LinkedList::new(),
        iter: into_iter_able.into_iter(),
        pred: pred,
        split_cnt: 0,
        maxsplit: maxsplit,
        iter_finished: false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_before(v, |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![0,1,2])), r.next());
        assert_eq!(Some(Ok(vec![3,4,5])), r.next());
        assert_eq!(Some(Ok(vec![6,7,8])), r.next());
        assert_eq!(Some(Ok(vec![9])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_before(v, |x|{x%3==0}, 2);
        assert_eq!(Some(Ok(vec![0,1,2])), r.next());
        assert_eq!(Some(Ok(vec![3,4,5])), r.next());
        assert_eq!(Some(Ok(vec![6,7,8,9])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![0,0,0];
        let mut r = splite_before(v, |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![1,1,1];
        let mut r = splite_before(v, |x|{x%3==0}, -1);
        assert_eq!(Some(Ok(vec![1,1,1])), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());
    }
}