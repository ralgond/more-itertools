use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug, Clone)]
pub struct SplitAt<I: Iterator> {
    ret_buf: VecDeque<Vec<I::Item>>,
    buf: Vec<I::Item>,
    iter: I,
    pred: fn(&I::Item) -> bool,
    maxsplit: i64,
    keep_separator: bool,
    splited: i64,
    iter_finished: bool,
    found_sep: bool
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
                    let mut empty_vec = Vec::new();
                    std::mem::swap(&mut empty_vec, &mut self.buf);
                    self.ret_buf.push_back(empty_vec);

                    let mut sep_vec = Vec::new();
                    sep_vec.push(v);
                    self.ret_buf.push_back(sep_vec);
                }
            }
        }

        if self.iter_finished && self.ret_buf.len() > 0 {
            let last_buf = self.ret_buf.back().unwrap();
            if last_buf.len() == 1 && (self.pred)(last_buf.get(0).unwrap()) {
                self.ret_buf.push_back(Vec::new());
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
                        return Some(v);
                    } else {
                        if v.len() == 1 && (self.pred)(v.get(0).unwrap()) {
                            continue;
                        } else {
                            return Some(v);
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
        iter_finished: false,
        found_sep: false
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