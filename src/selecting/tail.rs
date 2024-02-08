use std::collections::VecDeque;

pub struct Tail<I>
where 
    I: Iterator,
    I::Item: Clone
 {
    buf: VecDeque<I::Item>,
    iter: I,
    n: usize
}

impl<I> Iterator for Tail<I> 
where 
    I: Iterator,
    I::Item: Clone
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }
        loop {
            let nv = self.iter.next();
            match nv {
                None => {
                    return self.buf.pop_front();                  
                },
                Some(v) => {
                    if self.buf.len() == self.n {
                        self.buf.pop_front();
                    }
                    self.buf.push_back(v);
                }
            }
        }
    }
}

pub fn tail<I>(iterable: I, n: usize) -> Tail<I::IntoIter>
where
    I: IntoIterator,
    I::Item: Clone
{
    Tail {
        buf: VecDeque::new(),
        iter: iterable.into_iter(),
        n: n
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];

        let mut t = tail(v, 3);
        assert_eq!(Some(3), t.next());
        assert_eq!(Some(4), t.next());
        assert_eq!(Some(5), t.next());
        assert_eq!(None, t.next());


        let v = vec![1,2,3,4,5];

        let mut t = tail(v, 7);
        assert_eq!(Some(1), t.next());
        assert_eq!(Some(2), t.next());
        assert_eq!(Some(3), t.next());
        assert_eq!(Some(4), t.next());
        assert_eq!(Some(5), t.next());
        assert_eq!(None, t.next());
    }
}