use std::collections::LinkedList;

struct SplitWhenOutputItem<T> {
    items: Vec<T>
}

pub struct SplitWhen<T> {
    ret_buf: LinkedList<SplitWhenOutputItem<T>>,
    iter: Box<dyn Iterator<Item = T>>,
    pred: fn(&T, &T) -> bool,
    maxsplit: i128,
    splited: usize,
    iter_finished: bool
}

impl<T> Iterator for SplitWhen<T>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.maxsplit == 0 {
            if self.iter_finished {
                return None;
            }
            let mut ret = Vec::new();
            loop {
                let _next = self.iter.next();
                match _next {
                    None => {
                        self.iter_finished = true;
                        return Some(ret);
                    },
                    Some(v) => {
                        ret.push(v);
                    }
                }
            }
        }

        loop {
            if self.iter_finished {
                break;
            }
            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    break;
                },
                Some(v) => {
                    if self.ret_buf.len() > 0 {
                        let items = &self.ret_buf.back_mut().unwrap().items;
                        if items.len() > 0 {
                            if self.maxsplit < 0 {
                                if (self.pred)(& v, & items.get(items.len()-1).unwrap()) {
                                    self.ret_buf.push_back(SplitWhenOutputItem{
                                        items: Vec::new()
                                    });
                                    self.ret_buf.back_mut().unwrap().items.push(v);
                                    break;
                                } else {
                                    self.ret_buf.back_mut().unwrap().items.push(v);
                                }
                            } else {
                                if (self.splited as i128) < self.maxsplit && (self.pred)(& v, & items.get(items.len()-1).unwrap()) {
                                    self.ret_buf.push_back(SplitWhenOutputItem{
                                        items: Vec::new()
                                    });
                                    self.ret_buf.back_mut().unwrap().items.push(v);

                                    self.splited += 1;
                                    break;
                                } else {
                                    self.ret_buf.back_mut().unwrap().items.push(v);
                                }
                            }
                            
                        } else {
                            self.ret_buf.back_mut().unwrap().items.push(v);
                        }
                        
                    } else {
                        self.ret_buf.push_back(SplitWhenOutputItem{
                            items: Vec::new()
                        });
                        self.ret_buf.back_mut().unwrap().items.push(v);
                    }
                }
            }
        }

        loop {
            let ret = self.ret_buf.pop_front();
            match ret {
                None => { return None; }
                Some(v) => {
                    return Some(v.items);
                }
            }
        }
    }
}

pub fn split_when<T>(iter: Box<dyn Iterator<Item = T>>, pred: fn(&T, &T)->bool, maxsplit: i128) -> Box<dyn Iterator<Item = Vec<T>>>
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
        items: Vec::new()
    });

    return Box::new(ret);
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(iter_from_vec(v), |x, y| { y > x }, -1);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3], vec![2, 5], vec![2, 4], vec![2]], ret);


        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(iter_from_vec(v), |x, y| { y > x }, 2);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3], vec![2, 5], vec![2, 4, 2]], ret);

        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(iter_from_vec(v), |x, y| { y > x }, 10);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3], vec![2, 5], vec![2, 4], vec![2]], ret);

        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(iter_from_vec(v), |x, y| { y > x }, 0);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3, 2, 5, 2, 4, 2]], ret);

        let v = vec![1,2];
        let sw = split_when(iter_from_vec(v), |x, y| { y > x }, -1);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1,2]], ret);

        let v = vec![2,1];
        let sw = split_when(iter_from_vec(v), |x, y| { y > x }, -1);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![2], vec![1]], ret);
    }
}