use std::{collections::VecDeque};

#[derive(Debug, Clone)]
pub struct Peekable<I: Iterator> {
    buf: VecDeque<I::Item>,
    iter: I
}

impl<I> Iterator for Peekable<I>
where
    I: Iterator, {
    type Item = <I as Iterator>::Item;

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

impl<I: Iterator> Peekable<I> {

    fn peek(&mut self) -> Option<&I::Item> {
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

    fn prepend(&mut self, args: Vec<I::Item>) {
        for item in args.into_iter().rev() {
            self.buf.push_front(item);
        }
    }
}

/// https://more-itertools.readthedocs.io/en/stable/_modules/more_itertools/more.html#peekable
pub fn peekable<I>(iterable: I) -> Peekable<I::IntoIter>
where
    I: IntoIterator,
{
    Peekable {
        buf: VecDeque::new(),
        iter: iterable.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_peek_1() {
        let mut ret = peekable([1,2,3]);
   
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
        let mut p = peekable([1, 2, 3]);
        p.prepend(vec![10, 11, 12]);

        match p.next() {
            Some(v) => { assert_eq!(v, 10); }
            None => {}
        }

        match p.peek() {
            Some(v) => { assert_eq!(*v, 11); }
            None => {}
        }

        let v = p.collect::<Vec<_>>();
        assert_eq!(v, [11, 12, 1, 2, 3]);
    }
}