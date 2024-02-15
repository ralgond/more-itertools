use crate::error::Error;
use crate::error;

#[derive(PartialEq)]
pub enum IncompleteType {
    Fill,
    Ignore,
    Strict
}

pub struct Grouper<T> {
    buf: Vec<T>,
    n: usize,
    incomplete: IncompleteType,
    fillvalue: Option<T>,
    cur: usize
}

impl<T> Iterator for Grouper<T> 
where
T: Clone
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.buf.len() {
            return None;
        }

        if self.n == 0 {
            return Some(Err(error::value_error("n should not be 0.".to_string())));
        }

        if self.incomplete == IncompleteType::Fill {
            match self.fillvalue {
                None => { return Some(Err(error::value_error("fillvalue should not be None when incomplete is Fill".to_string()))); },
                Some(_) => {}
            }
        }

        let mut ret = Vec::new();
        let mut cnt: usize = 0;
        while self.cur < self.buf.len() {
            if cnt == self.n {
                break;
            }
            cnt += 1;

            ret.push(self.buf[self.cur].clone());
            self.cur += 1;
        }

        if ret.len() < self.n {
            match self.incomplete {
                IncompleteType::Fill => {
                    for i in 0..(self.n - ret.len()) {
                        ret.push(self.fillvalue.as_ref().unwrap().clone());
                    }
                    return Some(Ok(ret));
                },
                IncompleteType::Ignore => {
                    return None;
                },
                IncompleteType::Strict => {
                    return Some(Err(error::value_error("length of buf should be divide by n".to_string())));
                }
            }
        } else {
            return Some(Ok(ret));
        }

        
    }
}

pub fn grouper<T>(buf: Vec<T>, n: usize, incomplete: IncompleteType, fillvalue: Option<T>) -> Grouper<T> {
    Grouper {
        buf: buf,
        n: n,
        incomplete: incomplete,
        fillvalue: fillvalue,
        cur: 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];

        let mut g = grouper(v, 3, IncompleteType::Fill, Some(0));
        assert_eq!(vec![1,2,3], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![10,0,0], g.next().unwrap().ok().unwrap());

        let v = vec![1,2,3,4,5,6,7,8,9,10];

        let mut g = grouper(v, 3, IncompleteType::Ignore, Some(0));
        assert_eq!(vec![1,2,3], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], g.next().unwrap().ok().unwrap());
        assert_eq!(None, g.next());


        let v = vec![1,2,3,4,5,6,7,8,9,10];

        let mut g = grouper(v, 3, IncompleteType::Strict, Some(0));
        assert_eq!(vec![1,2,3], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![4,5,6], g.next().unwrap().ok().unwrap());
        assert_eq!(vec![7,8,9], g.next().unwrap().ok().unwrap());
        match g.next() {
            Some(v) => {
                match v {
                    Ok(_) => { assert!(false); }
                    Err(_) => {assert!(true); }
                }
            }
            None => { assert!(false); }
        }
    }
}