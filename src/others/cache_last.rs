use std::{cell::RefCell, rc::Rc};

use crate::error::Error;

struct CacheLastInner<T> {
    iter: Box<dyn Iterator<Item = Result<T, Error>>>,
    iter_finished: bool,
    // iter_error: Option<Error>,
    last_item: Option<Result<T, Error>>,
    emmit_last_next_loop: bool
}

pub struct CacheLast<T> {
    cl_inner: Rc<RefCell<CacheLastInner<T>>>
}

struct CacheLastIter<T> {
    cl_inner: Rc<RefCell<CacheLastInner<T>>>
}

impl<T> Iterator for CacheLastIter<T> 
where T: Clone
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = self.cl_inner.borrow_mut();

        if inner.iter_finished {
            return None;
        }

        if inner.emmit_last_next_loop {
            inner.emmit_last_next_loop = false;
            if let Some(v) = &inner.last_item {
                return Some(v.clone());
            } else {
                return None;
            }
        }

        let _next = inner.iter.next();
        if let Some(_next2) = _next {
            inner.last_item = Some(_next2.clone());
            if let Err(_) = _next2 {
                inner.iter_finished = true;
            }
            return Some(_next2);
        } else {
            inner.iter_finished = true;
            inner.last_item = None;
            return None;
        }
    }
}

impl<T> CacheLast<T> 
where
T: Clone + 'static
{
    pub fn new(iter: Box<dyn Iterator<Item=Result<T,Error>>>) -> CacheLast<T> {
        let inner = CacheLastInner {
            iter,
            iter_finished: false,
            // iter_error: None,
            last_item: None,
            emmit_last_next_loop: false
        };

        let ret = CacheLast {
            cl_inner: Rc::new(RefCell::new(inner))
        };

        return ret;
    }

    pub fn iter(&mut self) -> Box<dyn Iterator<Item = Result<T,Error>>> {
        let ret0: Box<dyn Iterator<Item=Result<T,Error>>> = Box::new(CacheLastIter {
            cl_inner: Rc::clone(&self.cl_inner)
        });
        return ret0;
    }

    pub fn insert_last_to_head(&mut self) {
        let mut inner = self.cl_inner.borrow_mut();
        if inner.iter_finished {
            return;
        }

        if inner.last_item.is_none() {
            return;
        }

        inner.emmit_last_next_loop = true;
    }
}

pub fn cache_last<T>(iter: Box<dyn Iterator<Item = Result<T, Error>>>) -> CacheLast<T> 
where T: Clone + 'static
{
    CacheLast::new(iter)
}

#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = generate_okok_iterator(vec![1,2,3,4,5]);

        let mut cl = cache_last(v);

        let mut iter = cl.iter();

        assert_eq!(1, iter.next().unwrap().ok().unwrap());
        assert_eq!(2, iter.next().unwrap().ok().unwrap());
        assert_eq!(3, iter.next().unwrap().ok().unwrap());

        cl.insert_last_to_head();

        assert_eq!(3, iter.next().unwrap().ok().unwrap());
        assert_eq!(4, iter.next().unwrap().ok().unwrap());
        assert_eq!(5, iter.next().unwrap().ok().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test2() {
        let v = generate_okok_iterator(Vec::<i32>::new());

        let mut cl = cache_last(v);

        let mut iter = cl.iter();

        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}