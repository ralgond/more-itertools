use std::{cell::RefCell, rc::Rc};
use crate::error::{self, Error};
use crate::itertools::iter::iter_from_result_vec;
use crate::others::cache_last::cache_last;

struct BeforeAndAfterInner<T>
where
T: Clone
{
    before: Box<dyn Iterator<Item = Result<T,Error>>>,
    // cl: CacheLast<T>,
    cl_iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    err: Option<Error>
}

impl<T> BeforeAndAfterInner<T> 
where
T: Clone + 'static {
    pub fn new(iter: Box<dyn Iterator<Item = Result<T,Error>>>, 
                predicate: fn(item: &T) -> Result<bool, Error>) -> Self {
        let mut cl = cache_last(iter);
        let mut cl_iter = cl.iter();
        let mut before = Vec::<Result<T,Error>>::new();
        let mut err: Option<Error> = None;

        loop {
            if let Some(ret) = cl_iter.next() {
                if ret.is_err() {
                    err = Some(ret.err().unwrap().clone());
                    break;
                }

                let pred_result = predicate(&ret.clone().ok().unwrap());
                if pred_result.is_err() {
                    let pred_err = pred_result.err().unwrap();
                    err = Some(error::any_error(pred_err.kind(), "[before_and_after] ".to_string() + pred_err.message().unwrap()));
                    break;
                }

                if pred_result.ok().unwrap() {
                    before.push(ret);
                } else {
                    break;
                }
            } else {
                break;
            }
        } 

        cl.insert_last_to_head();
        
        let ret = BeforeAndAfterInner {
            before: iter_from_result_vec(before),
            //cl,
            cl_iter,
            err
        };

        return ret;
    }
}

pub struct BeforeAndAfter<T> 
where
T: Clone
{
    inner: Rc<RefCell<BeforeAndAfterInner<T>>>
}

impl<T> BeforeAndAfter<T> 
where
T: Clone + 'static
{
    pub fn new(iter: Box<dyn Iterator<Item = Result<T,Error>>>,
                predicate: fn(item: &T) -> Result<bool, Error>) -> Self {
        let inner = BeforeAndAfterInner::new(iter, predicate);

        let ret = BeforeAndAfter {
            inner: Rc::new(RefCell::new(inner))
        };

        return ret;
    }

    pub fn iter(&self) -> (Box<dyn Iterator<Item=Result<T,Error>>>, Box<dyn Iterator<Item=Result<T,Error>>>) {
        let ret_before = Box::new(BeforeCursor {
            err: self.inner.borrow_mut().err.clone(),
            iter_finished: false,
            inner: Rc::clone(&self.inner)
        });

        let ret_after = Box::new(AfterCursor {
            err: self.inner.borrow_mut().err.clone(),
            iter_finished: false,
            inner: Rc::clone(&self.inner)
        });

        return (ret_before, ret_after);
    }
}

struct BeforeCursor<T> 
where
T: Clone
{
    err: Option<Error>,
    iter_finished: bool,
    inner: Rc<RefCell<BeforeAndAfterInner<T>>>
}

struct AfterCursor<T> 
where
T: Clone
{
    err: Option<Error>,
    iter_finished: bool,
    inner: Rc<RefCell<BeforeAndAfterInner<T>>>
}

impl<T> Iterator for AfterCursor<T>    
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = self.inner.borrow_mut();

        if self.iter_finished {
            return None;
        }

        if let Some(_err) = &self.err {
            self.iter_finished = true;
            return Some(Err(_err.clone()));
        }

        let _next = inner.cl_iter.next();
        if let Some(_next_v) = _next {
            return Some(_next_v);
        } else {
            self.iter_finished = true;
            return None;
        }
    }
}

impl<T> Iterator for BeforeCursor<T>    
where
T: Clone
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = self.inner.borrow_mut();

        if self.iter_finished {
            return None;
        }

        if let Some(_err) = &self.err {
            self.iter_finished = true;
            return Some(Err(_err.clone()));
        }

        let _next = inner.before.next();
        if let Some(_next_v) = _next {
            return Some(_next_v);
        } else {
            self.iter_finished = true;
            return None;
        }
    }
}


pub fn before_and_after<T> (iter: Box<dyn Iterator<Item=Result<T,Error>>>, 
    predicate: fn(item: &T)->Result<bool,Error>) -> (Box<dyn Iterator<Item=Result<T,Error>>>, Box<dyn Iterator<Item=Result<T,Error>>>)
where
    T: Clone + 'static
{
    let baa = BeforeAndAfter::new(iter, predicate);
    return baa.iter();
}

#[cfg(test)]
mod tests {

    use crate::utils::{extract_value_from_result_vec, generate_okok_iterator, generate_okokerr_iterator};

    use super::*;

    #[test]
    fn test1() {
        let v1 = String::from("ABCdEfGhI");
        let (baa_before_iter, baa_after_iter) = before_and_after(generate_okok_iterator(v1.chars().collect()), |x: &char| { return Ok(x.is_ascii_uppercase()) });

        assert_eq! (vec!['A', 'B', 'C'], extract_value_from_result_vec(baa_before_iter.collect()).0);

        let v = baa_after_iter.collect::<Vec<_>>();
        assert_eq!(vec!['d', 'E', 'f', 'G', 'h', 'I'], extract_value_from_result_vec(v).0);
    }

    #[test]
    fn test2() {
        let v1 = String::from("ABC");
        let (baa_before_iter, baa_after_iter) = before_and_after(generate_okok_iterator(v1.chars().collect()), |x: &char| { return Ok(x.is_ascii_uppercase()) });

        assert_eq! (vec!['A', 'B', 'C'], extract_value_from_result_vec(baa_before_iter.collect()).0);

        let v = baa_after_iter.collect::<Vec<_>>();
        assert_eq!(Vec::<char>::new(), extract_value_from_result_vec(v).0);
    }

    #[test]
    fn test3() {
        let v1 = String::from("abc");
        let (baa_before_iter, baa_after_iter) = before_and_after(generate_okok_iterator(v1.chars().collect()), |x: &char| { return Ok(x.is_ascii_uppercase()) });

        assert_eq! (Vec::<char>::new(), extract_value_from_result_vec(baa_before_iter.collect()).0);

        let v = baa_after_iter.collect::<Vec<_>>();
        assert_eq!(vec!['a', 'b', 'c'], extract_value_from_result_vec(v).0);
    }

    #[test]
    fn test4() {
        let v1 = String::from("abc");
        let (baa_before_iter, baa_after_iter) = before_and_after(
            generate_okokerr_iterator(v1.chars().collect(), error::overflow_error("for test".to_string())), 
            |x: &char| { return Ok(x.is_ascii_uppercase()) });

        assert_eq! (Vec::<char>::new(), extract_value_from_result_vec(baa_before_iter.collect()).0);

        let v = baa_after_iter.collect::<Vec<_>>();
        let ret = extract_value_from_result_vec(v);
        assert!(ret.1.is_some());
        assert_eq!(vec!['a', 'b', 'c'], ret.0);
    }

    #[test]
    fn test5() {
        let v1 = String::from("ABC");
        let (baa_before_iter, baa_after_iter) = before_and_after(
            generate_okokerr_iterator(v1.chars().collect(), error::overflow_error("for test".to_string())), 
            |x: &char| { return Ok(x.is_ascii_uppercase()) });


        let ret = extract_value_from_result_vec(baa_before_iter.collect());
        assert_eq! (Vec::<char>::new(), ret.0);
        assert!(ret.1.is_some());

        let v = baa_after_iter.collect::<Vec<_>>();
        let ret = extract_value_from_result_vec(v);
        assert!(ret.1.is_some());
        assert_eq!(Vec::<char>::new(), ret.0);
    }

    #[test]
    fn test6() {
        let v1 = String::from("ABC");
        let (baa_before_iter, baa_after_iter) = before_and_after(
            generate_okokerr_iterator(v1.chars().collect(), error::overflow_error("for test".to_string())), 
            |_: &char| { return Err(error::overflow_error("for test".to_string())) });


        let ret = extract_value_from_result_vec(baa_before_iter.collect());
        assert_eq! (Vec::<char>::new(), ret.0);
        assert!(ret.1.is_some());
        println!("{:?}", ret);

        let v = baa_after_iter.collect::<Vec<_>>();
        let ret = extract_value_from_result_vec(v);
        assert!(ret.1.is_some());
        assert_eq!(Vec::<char>::new(), ret.0);
    }

}