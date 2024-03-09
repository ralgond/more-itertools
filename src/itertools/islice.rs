use crate::error::Error;
use crate::error;

pub struct Islice<T> {
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    start: usize,
    stop: usize,
    step: usize,
    cur: usize,
    skipped_start: bool,
    iter_finished: bool,
    emitted_first: bool
}

impl<T> Iterator for Islice<T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if self.stop <= self.start {
            self.iter_finished = true;
            return None;
        }

        let mut ret = None;

        if !self.skipped_start {
            while self.cur < self.start {
                ret = self.iter.next();
                self.cur += 1;
                if let Some(ref v_ret) = ret {
                    match v_ret {
                        Ok(_) => {
                            //return Some(Ok(ok_v_ret));
                            continue;
                        },
                        Err(err_v_ret) => {
                            self.iter_finished = true;
                            return Some(Err(err_v_ret.clone())); // upstream error
                        }
                    }
                } else {
                    self.iter_finished = true;
                    return None;
                }
            }
            self.skipped_start = true;
        } 

        //let mut ret = None;
        if self.emitted_first {
            if (usize::MAX - self.cur) < self.step {
                self.iter_finished = true;
                return Some(Err(error::overflow_error(String::from("[islice:cur overflow]"))));
            }
            self.cur += self.step;
            if self.cur >= self.stop {
                self.iter_finished = true;
                return None;
            }
            for _ in 0..self.step {
                ret = self.iter.next();
            }
        } else {
            ret = self.iter.next();
            self.emitted_first = true;
        }
        
        match ret {
            None => {return None;}
            Some(v) => { 
                match v {
                    Ok(ok_v) => {
                        return Some(Ok(ok_v))
                    },
                    Err(err_v) => {
                        return Some(Err(err_v));
                    }
                }
            }
        }

    }
}


pub fn islice<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, start: usize, stop: usize, step: usize) -> Box<dyn Iterator<Item=Result<T, Error>>>
where
    T: 'static
{
    Box::new(Islice {
        iter,
        start,
        stop,
        step,
        cur: 0,
        skipped_start: false,
        iter_finished: false,
        emitted_first: false
    })
}


#[cfg(test)]
mod tests {
    use crate::utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator};

    use super::*;

    #[test]
    fn test1() {
        let i = islice(generate_okok_iterator(vec![0,1,2,3,4]), 3, 10, 1);
        let ret = extract_value_from_result_vec(i.collect::<Vec<_>>());
        assert_eq!(vec![3,4], ret.0);

        let mut i2 = islice(generate_okok_iterator(vec![0,1,2,3,4,5,6]), 7, 10, 1);
        assert_eq!(None, i2.next());

        let v3  = vec![0,1,2,3,4,5,6,7,8,9,10];
        let i3 = islice(generate_okok_iterator(v3), 3, 11, 3);
        let ret = extract_value_from_result_vec(i3.collect::<Vec<_>>());
        assert_eq!(vec![3,6,9], ret.0);

    }

    #[test]
    fn test2() {
        let v3  = generate_okokerr_iterator(vec![0,1,2,3,4,5,6,7,8,9,10], error::overflow_error("[test]".to_string()));
        let mut i3 = islice(v3, 9, 15, 3);
        assert_eq!(Some(Ok(9)), i3.next());
        assert_eq!(error::Kind::OverflowError, i3.next().unwrap().err().unwrap().kind());
        assert_eq!(None, i3.next());
    }
}