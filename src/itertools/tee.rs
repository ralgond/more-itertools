use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[allow(dead_code)]
pub struct TeeInner<I: Iterator> {
    buf1: VecDeque<I::Item>,
    buf2: VecDeque<I::Item>,
    iter: I,
    iter_finished: bool
}

pub struct Tee<I: Iterator> {
    inner: Rc<RefCell<TeeInner<I>>>
}

impl<I> Tee<I> 
where
I: Iterator,
I::Item: Clone
{
    pub fn new(iterable: I) -> Tee<I> {
        let inner = TeeInner {
            buf1: VecDeque::new(),
            buf2: VecDeque::new(),
            iter: iterable,
            iter_finished: false
        };

        let ret = Tee {
            inner: Rc::new(RefCell::new(inner))
        };

        return ret;
    }

    pub fn iter(&self) -> (TeeCursor<I>, TeeCursor<I>) {
        let ret0 = TeeCursor {
            no: 1,
            inner: Rc::clone(&self.inner)
        };

        let ret1 = TeeCursor {
            no: 2,
            inner: Rc::clone(&self.inner)
        };

        return (ret0, ret1);
    }
}

pub struct TeeCursor<I> 
where
I: Iterator
{
    no: usize,
    inner: Rc<RefCell<TeeInner<I>>>
}

impl<I> Iterator for TeeCursor<I>
where
I: Iterator,
I::Item: Clone
{
    type Item = <I as Iterator>::Item;

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


pub fn tee<J>(buf: J) -> (TeeCursor<J>, TeeCursor<J>)
where
J: Iterator,
J::Item: Clone
{
    let t = Tee::new(buf);
    return t.iter();
}



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let (t1,t2) = tee(v.into_iter());
        assert_eq!(vec![1, 2, 3, 4, 5], t1.collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3, 4, 5], t2.collect::<Vec<_>>());

        let v = vec![1,2,3,4,5];
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

        let v = vec![1,2,3,4,5];
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

        let v = vec![1,2,3,4];
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