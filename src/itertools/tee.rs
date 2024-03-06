use std::{cell::RefCell, collections::VecDeque, rc::Rc};
use crate::error::Error;

#[allow(dead_code)]
struct TeeInner<T> 
where
T: Clone
{
    buf1: VecDeque<T>,
    buf2: VecDeque<T>,
    iter: Box<dyn Iterator<Item = Result<T, Error>>>,
    iter_finished: bool,
    iter_error: Option<Error>
}

impl<T> TeeInner<T> 
where
T: Clone
{
    pub fn new(iter: Box<dyn Iterator<Item = Result<T, Error>>>) -> TeeInner<T> {
        return TeeInner {
            buf1: VecDeque::new(),
            buf2: VecDeque::new(),
            iter,
            iter_finished: false,
            iter_error: None
        };
    }
}

pub struct Tee<T> 
where
T: Clone 
{
    inner: Rc<RefCell<TeeInner<T>>>
}

struct TeeCursor<T> 
where
T: Clone
{
    no: usize,
    inner: Rc<RefCell<TeeInner<T>>>
}

impl<T> Iterator for TeeCursor<T> 
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let _next = self.inner.borrow_mut().iter.next();
        match _next {
            None => {
                self.inner.borrow_mut().iter_finished = true;
            }, 
            Some(v) => {
                if v.is_ok() {
                    self.inner.borrow_mut().buf1.push_back(v.as_ref().ok().unwrap().clone());
                    self.inner.borrow_mut().buf2.push_back(v.as_ref().ok().unwrap().clone());
                } else {
                    self.inner.borrow_mut().iter_error = Some(v.err().unwrap());
                }
            }
        }
        if self.no == 1 {
            if self.inner.borrow_mut().iter_finished && self.inner.borrow_mut().buf1.len() == 0 {
                return None;
            } else {
                return Some(Ok(self.inner.borrow_mut().buf1.pop_front().unwrap()));
            }
        } else if self.no == 2 {
            if self.inner.borrow_mut().iter_finished && self.inner.borrow_mut().buf2.len() == 0 {
                return None;
            } else {
                return Some(Ok(self.inner.borrow_mut().buf2.pop_front().unwrap()));
            }
        } else {
            return None;
        }
    }
}

impl<T> Tee<T> 
where
T: Clone + 'static
{
    pub fn new(iter: Box<dyn Iterator<Item=Result<T,Error>>>) -> Tee<T> {
        let inner = TeeInner::new(iter);

        let ret = Tee {
            inner: Rc::new(RefCell::new(inner))
        };

        return ret;
    }

    pub fn iter(&self) -> (Box<dyn Iterator<Item=Result<T,Error>>>, Box<dyn Iterator<Item=Result<T,Error>>>) {
        let ret0: Box<dyn Iterator<Item=Result<T,Error>>> = Box::new(TeeCursor {
            no: 1,
            inner: Rc::clone(&self.inner)
        });

        let ret1: Box<dyn Iterator<Item=Result<T,Error>>> = Box::new(TeeCursor {
            no: 2,
            inner: Rc::clone(&self.inner)
        });

        return (ret0, ret1);
    }
}



pub fn tee<T>(iterator: Box<dyn Iterator<Item=Result<T,Error>>>) -> (Box<dyn Iterator<Item=Result<T,Error>>>, Box<dyn Iterator<Item=Result<T,Error>>>) 
where
T: Clone + 'static
{
    let t = Tee::new(iterator);
    return t.iter();
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::utils::extract_value_from_result_vec;
    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = generate_okok_iterator(vec![1,2,3,4,5]);
        let (t1,t2) = tee(v);
        let ret1 = extract_value_from_result_vec(t1.collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3, 4, 5], ret1.0);
        assert_eq!(vec![1, 2, 3, 4, 5], extract_value_from_result_vec(t2.collect::<Vec<_>>()).0);

        let v = generate_okok_iterator(vec![1,2,3,4,5]);
        let (mut t1, mut t2) = tee(v);
        assert_eq!(Some(1), t1.next().unwrap().ok());
        assert_eq!(Some(1), t2.next().unwrap().ok());
        assert_eq!(Some(2), t1.next().unwrap().ok());
        assert_eq!(Some(2), t2.next().unwrap().ok());
        assert_eq!(Some(3), t1.next().unwrap().ok());
        assert_eq!(Some(3), t2.next().unwrap().ok());
        assert_eq!(Some(4), t1.next().unwrap().ok());
        assert_eq!(Some(4), t2.next().unwrap().ok());
        assert_eq!(Some(5), t1.next().unwrap().ok());
        assert_eq!(Some(5), t2.next().unwrap().ok());
        assert_eq!(None, t1.next());
        assert_eq!(None, t2.next());

        let v = generate_okok_iterator(vec![1,2,3,4,5]);
        let (mut t1, mut t2) = tee(v);
        assert_eq!(Some(1), t1.next().unwrap().ok());
        assert_eq!(Some(2), t1.next().unwrap().ok());
        assert_eq!(Some(1), t2.next().unwrap().ok());
        assert_eq!(Some(2), t2.next().unwrap().ok());

        assert_eq!(Some(3), t1.next().unwrap().ok());
        assert_eq!(Some(4), t1.next().unwrap().ok());
        assert_eq!(Some(3), t2.next().unwrap().ok());
        assert_eq!(Some(4), t2.next().unwrap().ok());

        assert_eq!(Some(5), t1.next().unwrap().ok());
        assert_eq!(Some(5), t2.next().unwrap().ok());

        assert_eq!(None, t1.next());
        assert_eq!(None, t2.next());

        let v = generate_okok_iterator(vec![1,2,3,4]);
        let (mut t1, mut t2) = tee(v);
        assert_eq!(Some(1), t1.next().unwrap().ok());
        assert_eq!(Some(2), t1.next().unwrap().ok());
        assert_eq!(Some(3), t1.next().unwrap().ok());
        assert_eq!(Some(1), t2.next().unwrap().ok());
        assert_eq!(Some(2), t2.next().unwrap().ok());
        assert_eq!(Some(3), t2.next().unwrap().ok());

        assert_eq!(Some(4), t1.next().unwrap().ok());
        assert_eq!(Some(4), t2.next().unwrap().ok());

        assert_eq!(None, t1.next());
        assert_eq!(None, t2.next());
    }
}