use crate::error::Error;

pub struct Chain<T> {
    input: Vec<Box<dyn Iterator<Item=Result<T, Error>>>>,
    cur_idx: usize,
    iter_finished: bool,
    iter_err: Option<Error>
}

impl<T> Iterator for Chain<T>
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.iter_err.is_some() {
                return Some(Err(self.iter_err.as_ref().unwrap().clone()));
            }
    
            if self.cur_idx >= self.input.len() {
                self.iter_finished = true;
                return None;
            }
    
            let cur = self.input.get_mut(self.cur_idx).unwrap();
            let _next = cur.next();
            match _next {
                None => {
                    self.cur_idx += 1;
                    continue;
                },
                Some(v) => {
                    return Some(v);
                }
            }
        }
    }
}

pub fn chain<T: 'static>(input: Vec<Box<dyn Iterator<Item = Result<T, Error>>>>) -> Box<dyn Iterator<Item = Result<T, Error>>>  {
    Box::new(Chain {
        input,
        cur_idx: 0,
        iter_finished: false,
        iter_err: None
    })
}

#[cfg(test)]
mod tests {
    use crate::utils::{extract_value_from_result_vec, generate_okok_iterator};

    use super::*;

    #[test]
    fn test1() {
        let mut input = Vec::new();

        let v = vec![1,2,3];
        let ret1 = generate_okok_iterator(v);
        input.push(ret1);

        let v = vec![2,4,6];
        let ret2 = generate_okok_iterator(v);
        input.push(ret2);

        let chain = chain(input);

        let ret = extract_value_from_result_vec(chain.collect::<Vec<_>>());
        assert!(ret.1.is_none());
        assert_eq!(vec![1,2,3,2,4,6], ret.0);
    }
}