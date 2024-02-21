use std::fmt::Debug;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
struct SplitWhenOutputItem<I: Iterator> {
    items: Vec<I::Item>
    
}

#[derive(Debug, Clone)]
pub struct SplitWhen<I: Iterator> {
    ret_buf: LinkedList<SplitWhenOutputItem<I>>,
    iter: I,
    pred: fn(& I::Item, & I::Item) -> bool,
    maxsplit: i128,
    splited: usize,
    iter_finished: bool
}

impl<I: Iterator> Iterator for SplitWhen<I> 
where I::Item: Debug
{
    type Item = Vec<<I as Iterator>::Item>;

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

pub fn split_when<I>(iter: I, pred: fn(&I::Item, &I::Item)->bool, maxsplit: i128) -> SplitWhen<I>
where
    I: Iterator,
{
    let mut ret = SplitWhen {
        ret_buf: LinkedList::new(),
        iter: iter,
        pred: pred,
        maxsplit: maxsplit,
        splited: 0,
        iter_finished: false
    };

    ret.ret_buf.push_back(SplitWhenOutputItem {
        items: Vec::new()
    });

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(v.into_iter(), |x, y| { y > x }, -1);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3], vec![2, 5], vec![2, 4], vec![2]], ret);


        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(v.into_iter(), |x, y| { y > x }, 2);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3], vec![2, 5], vec![2, 4, 2]], ret);

        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(v.into_iter(), |x, y| { y > x }, 10);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3], vec![2, 5], vec![2, 4], vec![2]], ret);

        let v = vec![1, 2, 3, 3, 2, 5, 2, 4, 2];
        let sw = split_when(v.into_iter(), |x, y| { y > x }, 0);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1, 2, 3, 3, 2, 5, 2, 4, 2]], ret);

        let v = vec![1,2];
        let sw = split_when(v.into_iter(), |x, y| { y > x }, -1);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![1,2]], ret);

        let v = vec![2,1];
        let sw = split_when(v.into_iter(), |x, y| { y > x }, -1);
        let ret = sw.collect::<Vec<_>>();
        assert_eq!(vec![vec![2], vec![1]], ret);
    }
}