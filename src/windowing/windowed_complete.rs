use crate::error::Error;
use crate::error;

pub struct WindowedComplete<T>
where 
T: Clone + 'static
 {
    buf: Vec<T>,
    iter: Box<dyn Iterator<Item=Result<T,Error>>>,
    n: usize,
    cur: usize,
    iter_finished: bool,
    upstream_error: Option<Error>
}

impl<T> Iterator for WindowedComplete<T> 
where 
T: Clone + 'static
{
    type Item = Result<(Vec<T>, Vec<T>, Vec<T>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if self.n > self.buf.len() {
            self.iter_finished = true;
            return Some(Err(error::value_error("[windowed_complete:n must be <= len(seq)]".to_string())));
        }

        if let Some(v_upstream_error) = &self.upstream_error {
            self.iter_finished = true;
            return Some(Err(v_upstream_error.clone()));
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
pub fn windowed_complete<T>(iter: Box<dyn Iterator<Item=Result<T,Error>>>, n: usize) -> Box<dyn Iterator<Item=Result<(Vec<T>, Vec<T>, Vec<T>), Error>>>
where
T: Clone + 'static
{
    let mut ret = WindowedComplete {
        buf: Vec::new(),
        iter,
        n,
        cur: 0,
        iter_finished: false,
        upstream_error: None
    };

    loop {
        if let Some(item) = ret.iter.next() {
            match item {
                Ok(ok_item) => {
                    ret.buf.push(ok_item);
                },
                Err(err_item) => {
                    ret.upstream_error = Some(err_item);
                    break;
                }
            }
        } else {
            break;
        }
    }

    return Box::new(ret);
}

#[cfg(test)]
mod tests {
    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = generate_okok_iterator(vec![0,1,2,3,4,5,6]);

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