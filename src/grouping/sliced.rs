use crate::error;
use crate::error::Error;

pub struct Sliced<T> {
    buf: Vec<T>,
    n: usize, 
    strict: bool,
    cur: usize
}

impl<T> Sliced<T> {
    pub fn new(buf: Vec<T>, n: usize, strict: bool) -> Self {
        Sliced {
            buf: buf,
            n: n,
            strict: strict,
            cur: 0
        }
    }
}

impl<T> Iterator for Sliced<T> 
where
T: Clone
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.cur >= self.buf.len() {
            return None;
        }

        let mut end = self.cur + self.n;

        if end > self.buf.len() {
            end = self.buf.len();
        }

        let mut ret = Vec::new();

        for i in self.cur..end {
            ret.push(self.buf[i].clone());
        }

        self.cur += self.n;

        if self.strict && ret.len() < self.n {
            return Some(Err(error::value_error("vector.len is not divisible by n.".to_string())));
        }

        return Some(Ok(ret));
    }
}

pub fn sliced<T>(buf: Vec<T>, n: usize, strict: bool) -> Sliced<T>
where
T: Clone
{
    return Sliced::new(buf, n, strict);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut it = sliced(vec![1,2,3,4,5,6,7,8,9,10], 3, false);
        assert_eq!(vec![1,2,3], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![10], it.next().unwrap().ok().unwrap());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test2() {
        let mut it = sliced(vec![1,2,3,4,5,6,7,8,9,10,11], 3, true);
        assert_eq!(vec![1,2,3], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], it.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], it.next().unwrap().ok().unwrap());
        match it.next().unwrap() {
            Ok(_) => { assert!(false); },
            Err(_) => { assert!(true); }
        }
    }
}