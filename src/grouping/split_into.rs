use std::collections::LinkedList;

use crate::error::Error;

struct SplitIntoOutputItem<T> {
    items: Vec<T>,
    size: usize,
    finished: bool
}

pub struct SplitInto<T>
{
    ret_buf: LinkedList<SplitIntoOutputItem<T>>,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    sizes: Vec<usize>,
    iter_finished: bool
}

impl<T> Iterator for SplitInto<T>
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                if self.ret_buf.len() > 0 {
                    let front = self.ret_buf.pop_front();
                    return Some(Ok(front.unwrap().items));
                } else {
                    return None;
                }
            }

            if self.ret_buf.len() > 0 {
                let front = self.ret_buf.front_mut().unwrap();
                if front.items.len() == front.size {
                    front.finished = true;
                }

                if front.finished {
                    let front = self.ret_buf.pop_front();
                    return Some(Ok(front.unwrap().items));
                }
            }

            let _next = self.iter.next();
            if let Some(v) = _next {
                match v {
                    Ok(ok_v) => {
                        if self.ret_buf.len() == 0 {
                            continue;
                        }

                        let front = self.ret_buf.front_mut();
                        front.unwrap().items.push(ok_v);

                        continue;
                    },
                    Err(err_v) => { // upstream error
                        self.iter_finished = true;
                        return Some(Err(err_v));
                    }
                }
            } else {
                self.iter_finished = true;
                for item in self.ret_buf.iter_mut() {
                    item.finished = true;
                }
                continue;
            }
        }
    }
}

pub fn split_into<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, sizes: Vec<usize>) -> Box<dyn Iterator<Item = Result<Vec<T>,Error>>>
where
T: 'static
{
    let mut ret = SplitInto {
        ret_buf: LinkedList::new(),
        iter,
        sizes: sizes.clone(),
        iter_finished: false
    };

    for size in ret.sizes.iter() {
        ret.ret_buf.push_back(SplitIntoOutputItem {
            finished: false,
            size: *size,
            items: Vec::new()
        });
    }

    return Box::new(ret);
}


#[cfg(test)]
mod tests {
    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6];
        let sizes = vec![1,2,3];
        let si = split_into(generate_okok_iterator(v), sizes);
        let ret = si.collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![vec![1], vec![2, 3], vec![4, 5, 6]], ret2.0);

        let v = vec![1,2,3,4,5,6];
        let sizes = vec![2,3];
        let si = split_into(generate_okok_iterator(v), sizes);
        let ret = si.collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![vec![1, 2], vec![3, 4, 5]], ret2.0);

        let v = vec![1,2,3,4];
        let sizes = vec![1,2,3,4];
        let si = split_into(generate_okok_iterator(v), sizes);
        let ret = si.collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![vec![1], vec![2, 3], vec![4], vec![]], ret2.0);

        let v = vec![1,2,3,4];
        let sizes = vec![1,2,0,3,4];
        let si = split_into(generate_okok_iterator(v), sizes);
        let ret = si.collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![vec![1], vec![2, 3], vec![], vec![4], vec![]], ret2.0);
    }

    #[test]
    fn test1_error() {
        let v = vec![1,2,3,4,5,6];
        let sizes = vec![1,2,3];
        let si = split_into(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), sizes);
        let ret = si.collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![vec![1], vec![2, 3], vec![4, 5, 6]], ret2.0);
        assert_eq!(error::Kind::OverflowError, ret2.1.unwrap().kind());

        let v = vec![1,2,3,4,5,6];
        let sizes = vec![1,2,4];
        let si = split_into(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), sizes);
        let ret = si.collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![vec![1], vec![2, 3]], ret2.0);
        assert_eq!(error::Kind::OverflowError, ret2.1.unwrap().kind());
    }

}
