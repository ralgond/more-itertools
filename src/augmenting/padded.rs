use crate::error::Error;

struct Padded<T> {
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    iter_finished: bool,
    iter_count: usize,
    count: usize,
    fill_value: T,
    error: Option<Error>
}

impl <T> Padded<T>
where T: Clone + 'static {
    pub fn try_return_fill_value_or_none(&mut self) -> Option<Result<T,Error>> {
        if self.iter_count < self.count {
            self.iter_count += 1;
            return Some(Ok(self.fill_value.clone()));
        } else {
            return None
        }
    }
}

impl <T> Iterator for Padded<T>
where T: Clone + 'static
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            if self.error.is_some() {
                return None;
            }
            return self.try_return_fill_value_or_none();
        } else {
            if let Some(_next) = self.iter.next() {
                self.iter_count += 1;
                match _next {
                    Ok(ok_v) => { 
                        return Some(Ok(ok_v));
                    },
                    Err(err_v) => { // upstream error
                        self.iter_finished = true;
                        self.error = Some(err_v);
                        return Some(Err(self.error.as_ref().unwrap().clone()));
                    }
                }
            } else {
                self.iter_finished = true;
                return self.try_return_fill_value_or_none();
            }
        }
    }
}

pub fn padded<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, fill_value: T, count: usize) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where T: Clone + 'static
{
    Box::new(Padded {
        iter,
        iter_finished: false,
        iter_count: 0,
        count,
        fill_value,
        error: None
    })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{error, utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let p = padded(generate_okok_iterator(v), 0, 5);
        assert_eq!(vec![1,2,3,0,0], extract_value_from_result_vec(p.collect::<Vec<_>>()).0);

        let v = vec![1,2,3];
        let p = padded(generate_okok_iterator(v), 0, 2);
        assert_eq!(vec![1,2,3], extract_value_from_result_vec(p.collect::<Vec<_>>()).0);

        let v = vec![1,2,3];
        let p = padded(generate_okokerr_iterator(v,error::overflow_error("[test]".to_string())), 0, 2);
        let ret = extract_value_from_result_vec(p.collect::<Vec<_>>());
        assert_eq!(vec![1,2,3], ret.0);
        assert_eq!(error::Kind::OverflowError, ret.1.unwrap().kind());
    }
}

//     #[test]
//     fn test2() {
//         let v = vec![1,2,3];
//         let p = padded(iter_from_vec(v), 0, 5, true);
//         assert_eq!(vec![1,2,3,0,0], p.collect::<Vec<_>>());

//         let v = vec![1,2,3];
//         let p = padded(iter_from_vec(v), 0, 2, true);
//         assert_eq!(vec![1,2,3,0], p.collect::<Vec<_>>());

//         let v = vec![1,2,3,4];
//         let p = padded(iter_from_vec(v), 0, 2, true);
//         assert_eq!(vec![1,2,3,4], p.collect::<Vec<_>>());

//         let v = vec![1,2,3,4];
//         let p = padded(iter_from_vec(v), 0, 4, true);
//         assert_eq!(vec![1,2,3,4], p.collect::<Vec<_>>());
//     }
// }