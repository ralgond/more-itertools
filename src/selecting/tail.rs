use std::collections::VecDeque;

pub struct Tail<T>
{
    buf: VecDeque<T>,
    iter: Box<dyn Iterator<Item = T>>,
    n: usize
}

impl<T> Iterator for Tail<T> 
{
    type Item = T;

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

pub fn tail<T>(iter: Box<dyn Iterator<Item = T>>, n: usize) -> Box<dyn Iterator<Item = T>>
where
T: 'static
{
    Box::new(Tail {
        buf: VecDeque::new(),
        iter,
        n
    })
}

#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];

        let mut t = tail(iter_from_vec(v), 3);
        assert_eq!(Some(3), t.next());
        assert_eq!(Some(4), t.next());
        assert_eq!(Some(5), t.next());
        assert_eq!(None, t.next());


        let v = vec![1,2,3,4,5];

        let mut t = tail(iter_from_vec(v), 7);
        assert_eq!(Some(1), t.next());
        assert_eq!(Some(2), t.next());
        assert_eq!(Some(3), t.next());
        assert_eq!(Some(4), t.next());
        assert_eq!(Some(5), t.next());
        assert_eq!(None, t.next());
    }
}