use std::collections::LinkedList;

struct SplitAtOutputItem<T> {
    is_sep: bool,
    items: Vec<T>
}

pub struct SplitAt<T> {
    ret_buf: LinkedList<SplitAtOutputItem<T>>,
    iter: Box<dyn Iterator<Item = T>>,
    pred: fn(&T) -> bool,
    maxsplit: i64,
    keep_separator: bool,
    splited: i64,
    iter_finished: bool
}


impl<T> Iterator for SplitAt<T> 
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
                    // self.ret_buf.push_back(SplitAtOutputItem{
                    //     is_sep: true,
                    //     is_end: true,
                    //     items: vec![]
                    // });
                    break;
                },
                Some(v) => {
                    if self.maxsplit < 0 {
                        if (self.pred)(&v) {
                            self.ret_buf.push_back(SplitAtOutputItem{
                                is_sep: true,
                                items: vec![v]
                            });

                            self.ret_buf.push_back(SplitAtOutputItem{
                                is_sep: false,
                                items: vec![]
                            });

                            break;
                        } else {
                            self.ret_buf.back_mut().unwrap().items.push(v);
                        }
                    } else {
                        if self.splited < self.maxsplit && (self.pred)(&v) {
                            self.ret_buf.push_back(SplitAtOutputItem{
                                is_sep: true,
                                items: vec![v]
                            });

                            self.ret_buf.push_back(SplitAtOutputItem{
                                is_sep: false,
                                items: vec![]
                            });

                            self.splited += 1;

                            break;
                        } else {
                            self.ret_buf.back_mut().unwrap().items.push(v);
                        }
                    }
                }
            }
        }

        loop {
            let ret = self.ret_buf.pop_front();
            match ret {
                None => { return None; }
                Some(v) => {
                    if self.keep_separator {
                        return Some(v.items);
                    } else {
                        if v.is_sep {
                            continue;
                        } else {
                            return Some(v.items);
                        }
                    }
                }
            }
        }
    }
}

pub fn split_at<T>(iter: Box<dyn Iterator<Item = T>>, pred: fn(&T)->bool, maxsplit: i64, keep_separator: bool) -> Box<dyn Iterator<Item=Vec<T>>>
where
T: 'static
{
    let mut ret = SplitAt {
        ret_buf: LinkedList::new(),
        iter: iter,
        pred: pred,
        maxsplit: maxsplit,
        keep_separator: keep_separator,
        splited: 0,
        iter_finished: false
    };

    ret.ret_buf.push_back(SplitAtOutputItem {
        is_sep: false,
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
        let v = vec!['a','a','a'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, -1, true);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec![], vec!['a'], vec![], vec!['a'], vec![], vec!['a'], vec![]], ret);
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());

        let v = vec!['a','a','a'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![Vec::<char>::new(), vec![], vec![], vec![]], ret);


        let v = vec!['a','a','a'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, 0, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['a','a','a']], ret);

        let v = vec!['b', 'a','b', 'a', 'b', 'a'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b'], vec![]], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'a', 'b'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b'], vec!['b']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b', 'b'], vec!['b', 'c']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, -1, true);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['a'], vec!['b'], vec!['a'], vec!['b', 'b'], vec!['a'], vec!['b', 'c']], ret);
    }

    #[test]
    pub fn test2() {
        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, 2, true);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['a'], vec!['b'], vec!['a'], vec!['b', 'b', 'a', 'b', 'c']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, 2, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b', 'b', 'a', 'b', 'c']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, 10, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b', 'b'], vec!['b', 'c']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c', 'a'];
        let sa = split_at(iter_from_vec(v), |x| { *x == 'a' }, 10, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b', 'b'], vec!['b', 'c'], vec![]], ret);
    }
}