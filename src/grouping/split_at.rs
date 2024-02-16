use std::{collections::VecDeque, fmt::Debug};


#[derive(Debug, Clone)]
struct SplitAtOutputItem<I: Iterator> {
    is_sep: bool,
    items: Vec<I::Item>
}

#[derive(Debug, Clone)]
pub struct SplitAt<I: Iterator> {
    ret_buf: VecDeque<SplitAtOutputItem<I>>,
    buf: Vec<I::Item>,
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
        let _next = self.iter.next();
        match _next {
            None => { self.iter_finished = true },
            Some(v) => {
                if (self.pred)(&v) {
                    let mut empty_vec = SplitAtOutputItem {
                        is_sep: false,
                        items: Vec::new()
                    };
                    std::mem::swap(&mut empty_vec.items, &mut self.buf);
                    self.ret_buf.push_back(empty_vec);

                    let mut sep_vec = SplitAtOutputItem {
                        is_sep: true,
                        items: Vec::new()
                    };
                    sep_vec.items.push(v);
                    self.ret_buf.push_back(sep_vec);
                }
            }
        }

        if self.iter_finished && self.ret_buf.len() > 0 {
            let last_buf = self.ret_buf.back().unwrap();
            if last_buf.items.len() == 1 && last_buf.is_sep {
                self.ret_buf.push_back(SplitAtOutputItem {
                    is_sep: false,
                    items: Vec::new()
                });
            }
        }

        if self.ret_buf.len() == 0 {
            return None;
        }

        loop {
            let ret = self.ret_buf.pop_front();
            match ret {
                None => {
                    return None;
                },
                Some(v) => {
                    if self.keep_separator {
                        return Some(v.items);
                    } else {
                        if v.items.len() == 1 && v.is_sep {
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
    let ret = SplitAt {
        ret_buf: VecDeque::new(),
        buf: Vec::new(),
        iter: iter,
        pred: pred,
        maxsplit: maxsplit,
        keep_separator: keep_separator,
        splited: 0,
        iter_finished: false
    };

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
        println!("{:?}", ret);
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());
        // println!("{:?}", sa.next());

        let v = vec!['a','a','a'];
        let sa = split_at(v.into_iter(), |x| { *x == 'a' }, -1, false);
        let ret = sa.collect::<Vec<_>>();
        println!("{:?}", ret);
    }
}