use crate::error::Error;

pub fn iter_from_vec<T: 'static>(v: Vec<T>) -> Box<dyn Iterator<Item=T>> {
    return Box::new(v.into_iter());
}

struct IterFromResultVec<T> {
    v: Vec<Result<T,Error>>,
    cur: usize,
    iter_finished: bool
}

impl<T> Iterator for IterFromResultVec<T>
where T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if self.cur == self.v.len() {
            self.iter_finished = true;
            return None;
        }

        let ret = self.v.get(self.cur);
        self.cur += 1;
        if let Some(ret2) = ret {
            if ret2.is_err() {
                self.iter_finished = true;
            }
            return Some(ret2.clone());
        } else {
            return None;
        }
    }
}

pub fn iter_from_result_vec<T>(v: Vec<Result<T,Error>>) -> Box<dyn Iterator<Item=Result<T,Error>>> 
where T: Clone + 'static
{
    let ret = IterFromResultVec {
        v,
        cur: 0,
        iter_finished: false
    };

    return Box::new(ret);
}


#[cfg(test)]
mod tests {

    use crate::error;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![Ok(1),Ok(2),Err(error::overflow_error("for test".to_string())),Ok(3)];
        let mut iter = iter_from_result_vec(v);
        assert_eq!(Ok(1), iter.next().unwrap());
        assert_eq!(Ok(2), iter.next().unwrap());
        assert_eq!(error::Kind::OverflowError, iter.next().unwrap().err().unwrap().kind());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}