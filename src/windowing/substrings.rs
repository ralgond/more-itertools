

pub struct Substrings<T> 
where
T: Clone
{
    iter: Box<dyn Iterator<Item = T>>,
    substring_len: usize,
    cur: usize,
    vec: Vec<T>,
    first_iter_loop_finished: bool
}


impl<T> Iterator for Substrings<T> 
where 
T: Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.first_iter_loop_finished {
            match self.iter.next() {
                None => { 
                    self.first_iter_loop_finished = true;
                    break;
                },
                Some(v) => {
                    self.vec.push(v);
                }
            }
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
                return Some(ret);
            }
        }
    }
}


pub fn substrings<T>(iter: Box<dyn Iterator<Item = T>>) -> Box<dyn Iterator<Item = Vec<T>>>
where
T: Clone + 'static
{
    Box::new(Substrings {
        iter,
        substring_len: 1,
        cur: 0,
        vec: Vec::new(),
        first_iter_loop_finished: false
    })
}


#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4];
        let mut ss = substrings(iter_from_vec(v));

        assert_eq!(Some(vec![1]), ss.next());
        assert_eq!(Some(vec![2]), ss.next());
        assert_eq!(Some(vec![3]), ss.next());
        assert_eq!(Some(vec![4]), ss.next());

        assert_eq!(Some(vec![1,2]), ss.next());
        assert_eq!(Some(vec![2,3]), ss.next());
        assert_eq!(Some(vec![3,4]), ss.next());

        assert_eq!(Some(vec![1,2,3]), ss.next());
        assert_eq!(Some(vec![2,3,4]), ss.next());

        assert_eq!(Some(vec![1,2,3,4]), ss.next());

        assert_eq!(None, ss.next());
        assert_eq!(None, ss.next());
    }
}
