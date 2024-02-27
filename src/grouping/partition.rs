use std::rc::Rc;

use crate::sequence::Sequence;


pub struct PartitionInner<T> {
    buf: Box<dyn Sequence<T>>,
    pred: fn(&T) -> bool
}

pub struct Partition<T> {
    inner: Rc<PartitionInner<T>>
}

pub struct CursorFalse<T> {
    inner: Rc<PartitionInner<T>>,
    _next: usize
}

pub struct CursorTrue<T> {
    inner: Rc<PartitionInner<T>>,
    _next: usize
}

impl<T> Iterator for CursorFalse<T>
where
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self._next >= self.inner.buf.len() {
                return None;
            }
            let t = self.inner.buf.get(self._next).unwrap();
            self._next += 1;
            if !(self.inner.pred)(t) {
                return Some(t.clone());
            }
        }
    }
}

impl<T> Iterator for CursorTrue<T>
where
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self._next >= self.inner.buf.len() {
                return None;
            }
            let t = self.inner.buf.get(self._next).unwrap();
            self._next += 1;
            if (self.inner.pred)(t) {
                return Some(t.clone());
            }
        }
    }
}

impl<T> Partition<T> 
where T: Clone + 'static
{
    pub fn new(buf: Box<dyn Sequence<T>>, pred: fn(&T) -> bool) -> Partition<T> {
        let inner = PartitionInner {
            buf: buf,
            pred: pred
        };

        let ret = Partition {
            inner: Rc::new(inner)
        };

        return ret;
    }

    pub fn get_cursor(&self) -> (Box<dyn Iterator<Item = T>>, Box<dyn Iterator<Item = T>>){
        let cur_false: Box<dyn Iterator<Item = T>> = Box::new(CursorFalse {
            inner: Rc::clone(&self.inner),
            _next: 0
        });
        let cur_true: Box<dyn Iterator<Item = T>> = Box::new(CursorTrue {
            inner: Rc::clone(&self.inner),
            _next: 0
        });
        return (cur_false, cur_true);
    }
}

pub fn partition<T>(buf: Box<dyn Sequence<T>>, pred: fn(&T) -> bool) -> (Box<dyn Iterator<Item = T>>, Box<dyn Iterator<Item = T>>) 
where T: Clone + 'static
{
    let p = Partition::new(buf, pred);
    return p.get_cursor();
}


#[cfg(test)]
mod tests {
    use crate::sequence::create_seq_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];

        let (cur_false, cur_true) = partition(create_seq_from_vec(v), |x| {x % 2 == 1});
        assert_eq!(vec![2, 4, 6, 8, 10], cur_false.collect::<Vec<_>>());
        assert_eq!(vec![1, 3, 5, 7, 9], cur_true.collect::<Vec<_>>());
    }
}