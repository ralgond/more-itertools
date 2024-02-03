use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
pub struct Islice<I: Iterator> {
    start: usize,
    stop: usize,
    step: usize,
    iter: I,
    cur: usize,
    skipped_start: bool
}


impl<I: Iterator> Iterator for Islice<I> {
    type Item = Result<<I as Iterator>::Item, Error>;

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

pub fn islice<I>(iterable: I, start: usize, stop: usize, step: usize) -> Islice<I::IntoIter>
where
    I: IntoIterator,
{
    Islice {
        start: start,
        stop: stop,
        step: step,
        iter: iterable.into_iter(),
        cur: 0,
        skipped_start: false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v  = vec![0,1,2,3,4];
        let mut i = islice(v, 3, 10, 1);
        match i.next() {
            Some(v) => {
                match v {
                    Ok(ok) => { assert_eq!(3, ok); }
                    Err(_) => {}
                }
            },
            None => { assert!(false)}
        }

        match i.next() {
            Some(v) => {
                match v {
                    Ok(ok) => { assert_eq!(4, ok); }
                    Err(_) => {}
                }
            },
            None => { assert!(false)}
        }

        match i.next() {
            Some(_) => { assert!(false) },
            None => { assert!(true)}
        }


        let v2  = vec![0,1,2,3,4,5,6];
        let mut i2 = islice(v2, 7, 10, 1);
        match i2.next() {
            Some(_) => { assert!(false) },
            None => { assert!(true)}
        }

        let v3  = vec![0,1,2,3,4,5,6,7,8,9,10];
        let mut i3 = islice(v3, 3, 11, 3);
        match i3.next() {
            Some(v) => {
                match v {
                    Ok(ok) => { assert_eq!(3, ok); }
                    Err(_) => {}
                }
            },
            None => { assert!(false)}
        }
        match i3.next() {
            Some(v) => {
                match v {
                    Ok(ok) => { assert_eq!(6, ok); }
                    Err(_) => {}
                }
            },
            None => { assert!(false)}
        }
        match i3.next() {
            Some(v) => {
                match v {
                    Ok(ok) => { assert_eq!(9, ok); }
                    Err(_) => {}
                }
            },
            None => { assert!(false)}
        }
        match i3.next() {
            Some(_) => { assert!(false) },
            None => { assert!(true)}
        }
        match i3.next() {
            Some(_) => { assert!(false) },
            None => { assert!(true)}
        }
    }

    // #[test]
    // fn test2() {
    //     let v3  = vec![0,1,2,3,4,5,6,7,8,9,10];
    //     let mut i3 = islice(v3, 3, 11, 3);
    //     i3.collect()
    // }
}