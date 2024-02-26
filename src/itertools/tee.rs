use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[allow(dead_code)]
struct TeeInner<T> 
where
T: Clone
{
    buf1: VecDeque<T>,
    buf2: VecDeque<T>,
    iter: Box<dyn Iterator<Item = T>>,
    iter_finished: bool
}

impl<T> TeeInner<T> 
where
T: Clone
{
    pub fn new(iter: Box<dyn Iterator<Item = T>>) -> TeeInner<T> {
        return TeeInner {
            buf1: VecDeque::new(),
            buf2: VecDeque::new(),
            iter,
            iter_finished: false
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
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let _next = self.inner.borrow_mut().iter.next();
        match _next {
            None => {
                self.inner.borrow_mut().iter_finished = true;
            }, 
            Some(v) => {
                self.inner.borrow_mut().buf1.push_back(v.clone());
                self.inner.borrow_mut().buf2.push_back(v.clone());
            }
        }
        if self.no == 1 {
            if self.inner.borrow_mut().iter_finished && self.inner.borrow_mut().buf1.len() == 0 {
                return None;
            } else {
                return self.inner.borrow_mut().buf1.pop_front();
            }
        } else if self.no == 2 {
            if self.inner.borrow_mut().iter_finished && self.inner.borrow_mut().buf2.len() == 0 {
                return None;
            } else {
                return self.inner.borrow_mut().buf2.pop_front();
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
    pub fn new(iter: Box<dyn Iterator<Item=T>>) -> Tee<T> {
        let inner = TeeInner::new(iter);

        let ret = Tee {
            inner: Rc::new(RefCell::new(inner))
        };

        return ret;
    }

    pub fn iter(&self) -> (Box<dyn Iterator<Item=T>>, Box<dyn Iterator<Item=T>>) {
        let ret0: Box<dyn Iterator<Item=T>> = Box::new(TeeCursor {
            no: 1,
            inner: Rc::clone(&self.inner)
        });

        let ret1: Box<dyn Iterator<Item=T>> = Box::new(TeeCursor {
            no: 2,
            inner: Rc::clone(&self.inner)
        });

        return (ret0, ret1);
    }
}



pub fn tee<T: 'static>(iterator: Box<dyn Iterator<Item=T>>) -> (Box<dyn Iterator<Item=T>>,Box<dyn Iterator<Item=T>>) 
where
T: Clone
{
    let t = Tee::new(iterator);
    return t.iter();
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = iter_from_vec(vec![1,2,3,4,5]);
        let (t1,t2) = tee(v.into_iter());
        assert_eq!(vec![1, 2, 3, 4, 5], t1.collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3, 4, 5], t2.collect::<Vec<_>>());

        let v = iter_from_vec(vec![1,2,3,4,5]);
        let (mut t1, mut t2) = tee(v.into_iter());
        assert_eq!(Some(1), t1.next());
        assert_eq!(Some(1), t2.next());
        assert_eq!(Some(2), t1.next());
        assert_eq!(Some(2), t2.next());
        assert_eq!(Some(3), t1.next());
        assert_eq!(Some(3), t2.next());
        assert_eq!(Some(4), t1.next());
        assert_eq!(Some(4), t2.next());
        assert_eq!(Some(5), t1.next());
        assert_eq!(Some(5), t2.next());
        assert_eq!(None, t1.next());
        assert_eq!(None, t2.next());

        let v = iter_from_vec(vec![1,2,3,4,5]);
        let (mut t1, mut t2) = tee(v.into_iter());
        assert_eq!(Some(1), t1.next());
        assert_eq!(Some(2), t1.next());
        assert_eq!(Some(1), t2.next());
        assert_eq!(Some(2), t2.next());

        assert_eq!(Some(3), t1.next());
        assert_eq!(Some(4), t1.next());
        assert_eq!(Some(3), t2.next());
        assert_eq!(Some(4), t2.next());

        assert_eq!(Some(5), t1.next());
        assert_eq!(Some(5), t2.next());

        assert_eq!(None, t1.next());
        assert_eq!(None, t2.next());

        let v = iter_from_vec(vec![1,2,3,4]);
        let (mut t1, mut t2) = tee(v.into_iter());
        assert_eq!(Some(1), t1.next());
        assert_eq!(Some(2), t1.next());
        assert_eq!(Some(3), t1.next());
        assert_eq!(Some(1), t2.next());
        assert_eq!(Some(2), t2.next());
        assert_eq!(Some(3), t2.next());

        assert_eq!(Some(4), t1.next());
        assert_eq!(Some(4), t2.next());

        assert_eq!(None, t1.next());
        assert_eq!(None, t2.next());
    }
}