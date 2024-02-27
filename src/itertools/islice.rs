use crate::error::Error;
use crate::error;

pub struct Islice<T> {
    iter: Box<dyn Iterator<Item = T>>,
    start: usize,
    stop: usize,
    step: usize,
    cur: usize,
    skipped_start: bool
}


impl<T> Iterator for Islice<T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.skipped_start {
            if (usize::MAX - self.cur) < self.step {
                return Some(Err(error::value_error(String::from("cur overflow usize::MAX while iterating to stop."))));
            }
            self.cur += self.step;
            if self.cur >= self.stop {
                return None;
            }
            let mut ret = None;
            for _ in 0..self.step {
                ret = self.iter.next();
            }

            match ret {
                None => {return None;}
                Some(v) => { return  Some(Ok(v)); }
            }
        } else {
            let mut item = self.iter.next();
            while self.cur < self.start {
                if self.cur == usize::MAX {
                    return Some(Err(error::value_error(String::from("cur overflow usize::MAX while skiping to start."))));
                }
                self.cur += 1;
                
                match item {
                    None => { break; }
                    Some(_) => {}
                }

                item = self.iter.next();
            }
            
            self.skipped_start = true;

            match item {
                Some(v) => { return Some(Ok(v)); },
                None => { return None; }
            }
        }
    }
}

pub fn islice<T>(iter: Box<dyn Iterator<Item = T>>, start: usize, stop: usize, step: usize) -> Box<dyn Iterator<Item=Result<T, Error>>>
where
    T: 'static
{
    Box::new(Islice {
        iter,
        start: start,
        stop: stop,
        step: step,
        cur: 0,
        skipped_start: false
    })
}


#[cfg(test)]
mod tests {
    use crate::{itertools::iter::iter_from_vec, utils::extract_value_from_result_vec};

    use super::*;

    #[test]
    fn test1() {
        let i = islice(iter_from_vec(vec![0,1,2,3,4]), 3, 10, 1);
        let ret = extract_value_from_result_vec(i.collect::<Vec<_>>());
        assert_eq!(vec![3,4], ret.0);

        let mut i2 = islice(iter_from_vec(vec![0,1,2,3,4,5,6]), 7, 10, 1);
        assert_eq!(None, i2.next());

        let v3  = vec![0,1,2,3,4,5,6,7,8,9,10];
        let i3 = islice(iter_from_vec(v3), 3, 11, 3);
        let ret = extract_value_from_result_vec(i3.collect::<Vec<_>>());
        assert_eq!(vec![3,6,9], ret.0);

    }

    // #[test]
    // fn test2() {
    //     let v3  = vec![0,1,2,3,4,5,6,7,8,9,10];
    //     let mut i3 = islice(v3, 3, 11, 3);
    //     i3.collect()
    // }
}