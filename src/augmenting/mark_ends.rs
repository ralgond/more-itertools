use std::collections::VecDeque;
use crate::error::Error;

struct MarkEndsOutputItem<T> {
    sentinel: bool,
    item: Option<T>
}

pub struct MarkEnds<T> 
{
    iter: Box<dyn Iterator<Item=Result<T,Error>>>,
    emitted_head: bool,
    buffer: VecDeque<MarkEndsOutputItem<T>>,
    iter_finished: bool
}


impl<T> Iterator for MarkEnds<T>
{
    type Item = Result<(bool, bool, T), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.buffer.len() == 1 {
                if self.buffer.front().unwrap().sentinel {
                    self.buffer.pop_front();
                    return None;
                }
            } else if self.buffer.len() == 2 {
                if self.buffer.back().unwrap().sentinel {
                    let ret = self.buffer.pop_front().unwrap();
                    self.buffer.pop_front(); // pop sentinel
                    if self.emitted_head {
                        return Some(Ok((false, true, ret.item.unwrap())));
                    } else {
                        self.emitted_head = true;
                        return Some(Ok((true, true, ret.item.unwrap())));
                    }
                } else {
                    // tow (false,false)
                    let ret = self.buffer.pop_front().unwrap();
                    if self.emitted_head {
                        return Some(Ok((false, false, ret.item.unwrap())));
                    } else {
                        self.emitted_head = true;
                        return Some(Ok((true, false, ret.item.unwrap())));
                    }
                }
            }

            if self.iter_finished {
                return None;
            }
    
            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    self.buffer.push_back(MarkEndsOutputItem{
                        sentinel: true,
                        item: None
                    });
                } Some(v) => {
                    match v {
                        Ok(ok_v) => {
                            self.buffer.push_back(MarkEndsOutputItem{
                                sentinel: false,
                                item: Some(ok_v)
                            });
                        },
                        Err(err_v) => {
                            self.iter_finished = true;
                            return Some(Err(err_v));
                        }
                    }
                }
            }
        }
    }
}

pub fn mark_ends<T>(iter: Box<dyn Iterator<Item=Result<T,Error>>>) -> Box<dyn Iterator<Item=Result<(bool,bool,T),Error>>>
where T: 'static
{
    Box::new(MarkEnds {
        iter: iter,
        buffer: VecDeque::new(),
        emitted_head: false,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {

    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let me = mark_ends(generate_okok_iterator(v));
        assert_eq!(vec![(true, false, 1), (false, false, 2), (false, true, 3)], extract_value_from_result_vec(me.collect::<Vec<_>>()).0);

        let v = vec![1,2];
        let me = mark_ends(generate_okok_iterator(v));
        assert_eq!(vec![(true, false, 1), (false, true, 2)], extract_value_from_result_vec(me.collect::<Vec<_>>()).0);

        let v = vec![1];
        let me = mark_ends(generate_okok_iterator(v));
        assert_eq!(vec![(true, true, 1)], extract_value_from_result_vec(me.collect::<Vec<_>>()).0);

        let v = Vec::<(bool, bool, i32)>::new();
        let me = mark_ends(generate_okok_iterator(v));
        // println!("{:?}", me.collect::<Vec<_>>());
        assert_eq!(0, me.collect::<Vec<_>>().len());

        let v = vec![1,2,3];
        let mut me = mark_ends(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())));
        assert_eq!(Some(Ok((true, false, 1))), me.next());
        assert_eq!(Some(Ok((false, false, 2))), me.next());
        assert_eq!(error::Kind::OverflowError, me.next().unwrap().err().unwrap().kind());
        assert_eq!(None, me.next());
        assert_eq!(None, me.next());

    }
}