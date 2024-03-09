use crate::error::Error;

pub struct Substrings<T> 
where
T: Clone
{
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    substring_len: usize,
    cur: usize,
    vec: Vec<T>,
    upstream_error: Option<Error>,
    iter_finished: bool,
}


impl<T> Iterator for Substrings<T> 
where 
T: Clone
{
    type Item = Result<Vec<T>, Error>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.iter_finished {
            return None;
        }

        if let Some(v_upstream_error) = &self.upstream_error {
            self.iter_finished = true;
            return Some(Err(v_upstream_error.clone()));
        }

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
                for ele in self.vec[self.cur..self.cur+self.substring_len].iter() {
                    ret.push(ele.clone())
                }
                self.cur += 1;
                return Some(Ok(ret));
            }
        }
    }
}


pub fn substrings<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where
T: Clone + 'static
{
    let mut ret = Substrings {
        iter,
        substring_len: 1,
        cur: 0,
        vec: Vec::new(),
        upstream_error: None,
        iter_finished: false
    };

    loop {
        if let Some(item) = ret.iter.next() {
            match item {
                Ok(ok_item) => {
                    ret.vec.push(ok_item);
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
        let v = vec![1,2,3,4];
        let mut ss = substrings(generate_okok_iterator(v));

        assert_eq!(Some(vec![1]), ss.next().unwrap().ok());
        assert_eq!(Some(vec![2]), ss.next().unwrap().ok());
        assert_eq!(Some(vec![3]), ss.next().unwrap().ok());
        assert_eq!(Some(vec![4]), ss.next().unwrap().ok());

        assert_eq!(Some(vec![1,2]), ss.next().unwrap().ok());
        assert_eq!(Some(vec![2,3]), ss.next().unwrap().ok());
        assert_eq!(Some(vec![3,4]), ss.next().unwrap().ok());

        assert_eq!(Some(vec![1,2,3]), ss.next().unwrap().ok());
        assert_eq!(Some(vec![2,3,4]), ss.next().unwrap().ok());

        assert_eq!(Some(vec![1,2,3,4]), ss.next().unwrap().ok());

        assert_eq!(None, ss.next());
        assert_eq!(None, ss.next());
    }
}
