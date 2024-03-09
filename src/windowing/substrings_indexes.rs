use crate::error::Error;

pub struct SubstringsIndexes<T> {
    string_len: usize,
    substring_len: usize,
    cur: usize,
    vec: Vec<T>,
    reverse: bool,
    upstream_error: Option<Error>,
    iter_finished: bool,
}

impl<T> Iterator for SubstringsIndexes<T>
where
T: Clone
{
    type Item = Result<(Vec<T>, usize, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if let Some(v_upstream_error) = &self.upstream_error {
            self.iter_finished = true;
            return Some(Err(v_upstream_error.clone()));
        }

        if !self.reverse {
            loop {
                if self.substring_len > self.vec.len() {
                    return None;
                }
        
                if self.cur + self.substring_len > self.vec.len() {
                    self.cur = 0;
                    self.substring_len += 1;
                    continue;
                } else {
                    let mut ret = Vec::new();
                    for ele in &self.vec.as_slice()[self.cur..self.cur+self.substring_len] {
                        ret.push(ele.clone())
                    }
                    let cur = self.cur;
                    self.cur += 1;

                    return Some(Ok((ret, cur, cur+self.substring_len)));
                }
            }
        } else {
            loop {
                if self.substring_len <= 0 {
                    return None;
                }
    
                if self.cur == usize::MAX {
                    self.substring_len -= 1;
                    self.cur = self.string_len - self.substring_len;
                    continue;
                } else {
                    let mut ret = Vec::new();
                    for ele in &self.vec.as_slice()[self.cur..self.cur+self.substring_len] {
                        ret.push(ele.clone())
                    }
                    let cur = self.cur;
                    self.cur -= 1;

                    return Some(Ok((ret, cur, cur+self.substring_len)));
                }
            }
        }
    }
}


pub fn substrings_indexes<T>(mut iter: Box<dyn Iterator<Item = Result<T,Error>>>, reverse: bool) -> Box<dyn Iterator<Item = Result<(Vec<T>,usize,usize),Error>>> 
where
T: Clone + 'static
{
    let mut upstream_error: Option<Error> = None;

    let mut vec = Vec::new();
    loop {
        if let Some(item) = iter.next() {
            match item {
                Ok(ok_item) => {
                    vec.push(ok_item);
                },
                Err(err_item) => {
                    upstream_error = Some(err_item);
                    break;
                }
            }
        } else {
            break;
        }
    }

    let mut ret = SubstringsIndexes {
        string_len: vec.len(),
        substring_len: 1,
        cur: 0,
        vec,
        reverse,
        upstream_error,
        iter_finished: false,
    };

    if ret.reverse {
        ret.substring_len = ret.string_len;
        ret.cur = ret.string_len - ret.substring_len;
    }

    return Box::new(ret);
}

#[cfg(test)]
mod tests {
    use crate::{error, utils::{generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        let v = generate_okok_iterator("more".to_string().chars().collect());
        let mut ssi = substrings_indexes(v, false);

        assert_eq!(Some((vec!['m'], 0 as usize, 1 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['o'], 1 as usize, 2 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['r'], 2 as usize, 3 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['e'], 3 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['m', 'o'], 0 as usize, 2 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['o', 'r'], 1 as usize, 3 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['r', 'e'], 2 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['m', 'o', 'r'], 0 as usize, 3 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['o', 'r', 'e'], 1 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['m', 'o', 'r', 'e'], 0 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(None, ssi.next());
    }

    #[test]
    fn test2() {
        let v = generate_okok_iterator("more".to_string().chars().collect());
        let mut ssi = substrings_indexes(v, true);

        assert_eq!(Some((vec!['m', 'o', 'r', 'e'], 0 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['o', 'r', 'e'], 1 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['m', 'o', 'r'], 0 as usize, 3 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['r', 'e'], 2 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['o', 'r'], 1 as usize, 3 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['m', 'o'], 0 as usize, 2 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['e'], 3 as usize, 4 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['r'], 2 as usize, 3 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['o'], 1 as usize, 2 as usize)), ssi.next().unwrap().ok());
        assert_eq!(Some((vec!['m'], 0 as usize, 1 as usize)), ssi.next().unwrap().ok());
        assert_eq!(None, ssi.next());
    }

    #[test]
    fn test3() {
        let v = generate_okokerr_iterator("more".to_string().chars().collect(), error::overflow_error("[test]".to_string()));
        let mut ssi = substrings_indexes(v, true);
        assert_eq!(error::Kind::OverflowError, ssi.next().unwrap().err().unwrap().kind());
    }
}