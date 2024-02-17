use std::{collections::{LinkedList, VecDeque}, fmt::Debug};


#[derive(Debug, Clone)]
struct SplitAtOutputItem<I: Iterator> {
    is_sep: bool,
    is_end: bool,
    items: Vec<I::Item>
}

#[derive(Debug, Clone)]
pub struct SplitAt<I: Iterator> {
    ret_buf: LinkedList<SplitAtOutputItem<I>>,
    iter: I,
    pred: fn(&I::Item) -> bool,
    maxsplit: i64,
    keep_separator: bool,
    splited: i64,
    iter_finished: bool
}


impl<I: Iterator> Iterator for SplitAt<I> 
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
                                is_end: false,
                                items: vec![v]
                            });

                            self.ret_buf.push_back(SplitAtOutputItem{
                                is_sep: false,
                                is_end: false,
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
                                is_end: false,
                                items: vec![v]
                            });

                            self.ret_buf.push_back(SplitAtOutputItem{
                                is_sep: false,
                                is_end: false,
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

pub fn split_at<I>(iter: I, pred: fn(&I::Item)->bool, maxsplit: i64, keep_separator: bool) -> SplitAt<I>
where
    I: Iterator,
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
        is_end: false,
        items: Vec::new()
    });

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec!['a','a','a'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, true);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec![], vec!['a'], vec![], vec!['a'], vec![], vec!['a'], vec![]], ret);
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());

        let v = vec!['a','a','a'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![Vec::<char>::new(), vec![], vec![], vec![]], ret);


        let v = vec!['a','a','a'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, 0, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['a','a','a']], ret);

        let v = vec!['b', 'a','b', 'a', 'b', 'a'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b'], vec![]], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'a', 'b'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b'], vec!['b']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b', 'b'], vec!['b', 'c']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, true);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['a'], vec!['b'], vec!['a'], vec!['b', 'b'], vec!['a'], vec!['b', 'c']], ret);
    }

    #[test]
    pub fn test2() {
        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, 2, true);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['a'], vec!['b'], vec!['a'], vec!['b', 'b', 'a', 'b', 'c']], ret);

        let v = vec!['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b', 'c'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, 2, false);
        let ret = sa.collect::<Vec<_>>();
        assert_eq!(vec![vec!['b'], vec!['b'], vec!['b', 'b', 'a', 'b', 'c']], ret);
    }
}