use std::collections::VecDeque;

pub struct Peekable<T> {
    buf: VecDeque<T>,
    iter: Box<dyn Iterator<Item = T>>
}

impl<T> Iterator for Peekable<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == 0 {
            match self.iter.next() {
                Some(first_ele) => { Some(first_ele) },
                None => { None }
            }
        } else {
            let ret = self.buf.pop_front();
            return ret;
        }
    }
}

impl<T> Peekable<T> {

    pub fn peek(&mut self) -> Option<&T> {
        match self.buf.pop_front() {
            None => {
                match self.iter.next() {
                    None => { return None; },
                    Some(next_ele) => { 
                        self.buf.push_back(next_ele); 
                    }
                }
            }
            Some(front) => {
                self.buf.push_front(front);
            }
        }

        return self.buf.front();
    }

    pub fn prepend(&mut self, args: Vec<T>) {
        for item in args.into_iter().rev() {
            self.buf.push_front(item);
        }
    }
}

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#peekable
pub fn peekable<T>(iter: Box<dyn Iterator<Item = T>>) -> Peekable<T>
where
T: 'static
{
    Peekable {
        buf: VecDeque::new(),
        iter
    }
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1_peek_1() {
        let mut ret = peekable(iter_from_vec(vec![1,2,3]));
   
        assert_eq!(ret.peek(), Some(&1));
        assert_eq!(ret.peek(), Some(&1));
        ret.next();
        assert_eq!(ret.peek(), Some(&2));
        assert_eq!(ret.peek(), Some(&2));

        ret.next();
        ret.next();
        ret.next();
        ret.next();
        assert_eq!(ret.peek(), None);
        assert_eq!(ret.peek(), None);
    }

    #[test]
    fn test2_prepend_1() {
        let mut p = peekable(iter_from_vec(vec![1, 2, 3]));
        p.prepend(vec![10, 11, 12]);

        assert_eq!(p.next().unwrap(), 10);
        assert_eq!(p.peek().unwrap(), &11);

        let v = p.collect::<Vec<_>>();
        assert_eq!(v, [11, 12, 1, 2, 3]);
    }
}