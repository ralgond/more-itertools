use crate::error:: Error;

pub struct RepeatEach<T> 
where
T: Clone
{
    // cl: CacheLast<T>,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    n: usize,
    emit_count: usize,
    iter_finished: bool,
    last_item: Option<Result<T, Error>>
}


impl<T> Iterator for RepeatEach<T>
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.emit_count < self.n {
                self.emit_count += 1;
                return self.last_item.clone();
            } else {
                self.last_item = self.iter.next();
                if self.last_item.is_none() {
                    self.emit_count = 0;
                    self.iter_finished = true;
                    return None;
                } else {
                    match self.last_item.as_ref().unwrap() {
                        Ok(_) => {
                            self.emit_count = 0;
                            continue;
                        },
                        Err(err_v) => {
                            self.iter_finished = true;
                            return Some(Err(err_v.clone()));
                        }
                    }
                }
            }
        }
    }
}

pub fn repeat_each<T>(mut iter: Box<dyn Iterator<Item=Result<T, Error>>>, n: usize) -> Box<dyn Iterator<Item=Result<T, Error>>>
where
T: Clone + 'static
{
    let last_item = iter.next();

    Box::new(RepeatEach {
        iter,
        n,
        emit_count: 0,
        iter_finished: false,
        last_item 
    })
}

#[cfg(test)]
mod tests {

    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let ret = repeat_each(generate_okok_iterator(v), 3).collect::<Vec<_>>();
        assert_eq!(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], extract_value_from_result_vec(ret).0);

        let v = vec![1,2,3];
        let ret = repeat_each(generate_okok_iterator(v), 0).collect::<Vec<_>>();
        assert_eq!(0, ret.len());

        let v = vec![1,2,3];
        let ret = repeat_each(generate_okokerr_iterator(v, error::overflow_error("[test]".to_string())), 3).collect::<Vec<_>>();
        let ret2 = extract_value_from_result_vec(ret);
        assert_eq!(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], ret2.0);
        assert_eq!(error::Kind::OverflowError, ret2.1.unwrap().kind());
    }
}