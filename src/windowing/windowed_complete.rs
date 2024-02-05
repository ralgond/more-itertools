use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
pub struct WindowedComplete<I>
where 
    I: Iterator,
    I::Item: Clone
 {
    buf: Vec<I::Item>,
    iter: I,
    n: usize,
    cur: usize
}

impl<I> Iterator for WindowedComplete<I> 
where 
    I: Iterator,
    I::Item: Clone
{
    type Item = Result<(Vec<<I as Iterator>::Item>, Vec<<I as Iterator>::Item>, Vec<<I as Iterator>::Item>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > self.buf.len() {
            return Some(Err(error::value_error("n must be <= len(seq)".to_string())));
        }

        if self.cur + self.n > self.buf.len() {
            return None;
        }

        let mut beginning = Vec::new();
        for v in self.buf[0..self.cur].iter() {
            beginning.push(v.clone());
        }

        let mut middle = Vec::new();
        for v in self.buf[self.cur..self.cur+self.n].iter() {
            middle.push(v.clone());
        }

        let mut end = Vec::new();
        for v in self.buf[(self.cur+self.n)..self.buf.len()].iter() {
            end.push(v.clone());
        }

        self.cur += 1;

        return Some(Ok((beginning, middle, end)));
    }
}


/// https://more-itertools.readthedocs.io/en/v10.2.0/api.html#more_itertools.windowed_complete
pub fn windowed_complete<I>(iterable: I, n: usize) -> WindowedComplete<I::IntoIter>
where
    I: IntoIterator,
    I::Item: Clone
{
    let mut ret = WindowedComplete {
        buf: Vec::new(),
        iter: iterable.into_iter(),
        n: n,
        cur: 0
    };

    loop {
        let item = ret.iter.next();
        match item {
            None => { break; }
            Some(v) => {
                ret.buf.push(v);
            }
        }
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4,5,6];

        let mut wc = windowed_complete(v, 3);

        match wc.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2.0, vec![]);
                        assert_eq!(v2.1, vec![0, 1, 2]);
                        assert_eq!(v2.2, vec![3, 4, 5, 6]);
                    }
                    Err(_) => { assert!(false); }
                }
            }
        }

        match wc.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2.0, vec![0]);
                        assert_eq!(v2.1, vec![1, 2, 3]);
                        assert_eq!(v2.2, vec![4, 5, 6]);
                    }
                    Err(_) => { assert!(false); }
                }
            }
        }

        match wc.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2.0, vec![0, 1]);
                        assert_eq!(v2.1, vec![2, 3, 4]);
                        assert_eq!(v2.2, vec![5, 6]);
                    }
                    Err(_) => { assert!(false); }
                }
            }
        }

        match wc.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2.0, vec![0, 1, 2]);
                        assert_eq!(v2.1, vec![3, 4, 5]);
                        assert_eq!(v2.2, vec![6]);
                    }
                    Err(_) => { assert!(false); }
                }
            }
        }

        match wc.next() {
            None => { assert!(false); }
            Some(v) => {
                match v {
                    Ok(v2) => {
                        assert_eq!(v2.0, vec![0, 1, 2, 3]);
                        assert_eq!(v2.1, vec![4, 5, 6]);
                        assert_eq!(v2.2, vec![]);
                    }
                    Err(_) => { assert!(false); }
                }
            }
        }

        match wc.next() {
            None => { assert!(true); }
            Some(_) => { assert!(false); }
        }
    }
}