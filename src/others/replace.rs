use std::collections::{LinkedList, VecDeque};
use crate::error;
use crate::error::Error;

struct ReplaceOutputItem<T> {
    items: VecDeque<T>
}

impl<T> ReplaceOutputItem<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    pub fn push_back(&mut self, item: T) {
        self.items.push_back(item)
    }
    pub fn len(&self) -> usize {
        return self.items.len();
    }
}

pub struct Replace<I: Iterator> {
    output_item_list: LinkedList<ReplaceOutputItem<I::Item>>,
    query: Vec<I::Item>,
    substitutes: Vec<I::Item>,
    buf: VecDeque<I::Item>,
    iter: I,
    iter_finished: bool
}

impl<I: Iterator> Iterator for Replace<I> 
where <I as Iterator>::Item: PartialEq + Clone
{
    type Item = Result<<I as Iterator>::Item, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.query.len() == 0 {
            if self.iter_finished {
                return None;
            }
            let ret = self.iter.next();
            match ret {
                None => {
                    self.iter_finished = true; 
                    return None; 
                }
                Some(v) => {
                    return Some(Ok(v));
                }
            }
        }

        loop {
            if self.output_item_list.len() > 0 {
                if let Some(replace_output_item_front) = self.output_item_list.front_mut() {
                    let front = replace_output_item_front.pop_front();
                    if replace_output_item_front.len() == 0 {
                        self.output_item_list.pop_front();
                    }
                    if let Some(v) = front {
                        return Some(Ok(v));
                    } else {
                        continue;
                    }
                }               
            }

            if self.iter_finished {
                return None;
            }

            // assert!(self.output_item_list.len() == 0);

            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    if self.output_item_list.len() == 0 {
                        self.output_item_list.push_back(ReplaceOutputItem{
                            items: VecDeque::new()
                        });
                    }
                    while self.buf.len() > 0 {
                        self.output_item_list.back_mut().unwrap().push_back(self.buf.pop_front().unwrap());
                    }
                },
                Some(v) => {
                    self.buf.push_back(v);

                    if self.buf.len() == self.query.len() {
                        
                        // is buf equals to query?
                        let mut is_equal = true;
                        for i in 0..self.query.len() {
                            if self.buf.get(i) != self.query.get(i) {
                                is_equal = false;
                                break;
                            }
                        }

                        self.output_item_list.push_back(ReplaceOutputItem{
                            items: VecDeque::new()
                        });

                        if is_equal {
                            for i in self.substitutes.iter() {
                                self.output_item_list.back_mut().unwrap().push_back(i.clone());
                            }
                            self.buf.clear();
                        } else {
                            let front = self.buf.pop_front();
                            self.output_item_list.back_mut().unwrap().push_back(front.unwrap());
                        }
                    }
                }
            }
        }
    }
}

pub fn replace<I> (
    query: Vec<I::Item>,
    substitutes: Vec<I::Item>,
    into_iter_able: I
) -> Replace<I::IntoIter>
where 
I: IntoIterator
{
    Replace {
        output_item_list: LinkedList::new(),
        query: query,
        substitutes: substitutes,
        buf: VecDeque::new(),
        iter_finished: false,
        iter: into_iter_able.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let mut r = replace(vec![1,1,1], vec![3,4], vec![1,1,1,1,1]);
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(1)), r.next());
        assert_eq!(Some(Ok(1)), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());


        let mut r = replace(vec![1,2,5], vec![3,4], vec![0, 1, 2, 5, 0, 1, 2, 5]);
        assert_eq!(Some(Ok(0)), r.next());
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(0)), r.next());
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(None, r.next());

        let mut r = replace(vec![0,1,2], vec![3,4], vec![0, 1, 2, 5, 0, 1, 2, 5]);
        
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(5)), r.next());
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(5)), r.next());
        assert_eq!(None, r.next());
    }
}