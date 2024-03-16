use std::collections::{LinkedList, VecDeque};

use crate::error::{self, Error};

struct ReplaceOutputItem<T> {
    items: VecDeque<T>
}

struct Replace<T> {
    output_list: LinkedList<ReplaceOutputItem<T>>,
    cache: VecDeque<T>,
    iter: Box<dyn Iterator<Item = Result<T,Error>>>,
    iter_finished: bool,
    query: Vec<T>,
    sub: Vec<T>,
    error: Option<Error>,
    emitted_error: bool
}

impl<T> Replace<T> 
where T: Clone
{
    fn flush_sub(&mut self) {
        let mut item = ReplaceOutputItem {
            items: VecDeque::new()
        };

        for i in self.sub.iter() {
            item.items.push_back(i.clone());
        }

        self.cache.clear();

        self.output_list.push_back(item);
    }

    fn flush_cache(&mut self) {
        let mut item = ReplaceOutputItem {
            items: VecDeque::new()
        };

        std::mem::swap(&mut item.items, &mut self.cache);

        self.output_list.push_back(item);
    }

    fn push_and_pop(&mut self, v: T) -> Option<T> {
        if self.cache.len() < self.query.len() {
            self.cache.push_back(v);
            return None;
        } else {
            let ret = self.cache.pop_front();
            self.cache.push_back(v);
            return ret;
        }
    }

    fn push_back(&mut self, v: T) {
        if self.output_list.len() == 0 {
            self.output_list.push_back(ReplaceOutputItem{
                items: VecDeque::new()
            });
            self.output_list.back_mut().unwrap().items.push_back(v);
        }
    }
}

fn is_equals<T>(a: &VecDeque<T>, b: &Vec<T>) -> bool 
where T: PartialEq
{
    if a.len() != b.len() {
        return false;
    }

    let mut i: usize = 0;
    for a_v in a.iter() { 
        if let Some(b_v) = b.get(i) {
            if *a_v != *b_v {
                return false;
            }
        } else {
            return false;
        }
        i += 1;
    }

    return true;
}

impl<T> Iterator for Replace<T> 
where T: Clone + PartialEq + 'static
{
    type Item = Result<T,Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.output_list.len() > 0 {
                let front = self.output_list.front_mut();
                if front.as_ref().unwrap().items.len() == 0 {
                    self.output_list.pop_front();
                    continue;
                } else {
                    return Some(Ok(front.unwrap().items.pop_front().unwrap()));
                }
            }

            if self.error.is_some() {
                if !self.emitted_error {
                    self.emitted_error = true;
                    return Some(Err(self.error.as_ref().unwrap().clone()));
                }
            }
            
            if self.iter_finished {
                return None;
            }

            if is_equals(&self.cache, &self.query) {
                self.flush_sub();
            } else {
                if let Some(_next) = self.iter.next() {
                    match _next {
                        Ok(ok_next) => {
                            if let Some(poped_value) = self.push_and_pop(ok_next) {
                                self.push_back(poped_value);
                            } else {
                                // cache is not full
                            }
                        },
                        Err(err_next) => { // upstream error
                            self.flush_cache();
                            self.error = Some(err_next);
                            self.iter_finished = true;
                        }
                    }
                    
                } else {
                    self.flush_cache();
                    self.iter_finished = true;
                }
            }
        }
    }
}



pub fn replace<T>(mut iter: Box<dyn Iterator<Item = Result<T,Error>>>, query: Vec<T>, sub: Vec<T>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where T: Clone + PartialEq + 'static
{
    let mut iter_finished = false;
    let mut cache = VecDeque::new();
    let cache_size = query.len();
    let mut error: Option<Error> = None;
    if cache_size == 0 {
        error = Some(error::value_error("[replace:len of query should gt 0]".to_string()));
    } else {
        loop {
            if let Some(v) = iter.next() {
                match v {
                    Ok(ok_v) => {
                        cache.push_back(ok_v);
                        if cache.len() == cache_size {
                            break;
                        }
                    },
                    Err(err_v) => { // upstream error
                        iter_finished = true;
                        error = Some(err_v)
                    }
                }
            } else {
                iter_finished = true;
                break;
            }
        }
    }
    
    Box::new(Replace{
        output_list: LinkedList::new(),
        cache,
        iter,
        iter_finished,
        query,
        sub,
        error,
        emitted_error: false
    })

    //preload items into cache

}


#[cfg(test)]
mod tests {

    use crate::utils::{generate_okok_iterator, generate_okokerr_iterator};

    use super::*;

    #[test]
    fn test1() {
        let mut r = replace(generate_okok_iterator(vec![1,1,1,1,1]), vec![1,1,1], vec![3,4]);
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(1)), r.next());
        assert_eq!(Some(Ok(1)), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());


        let mut r = replace(generate_okok_iterator(vec![0, 1, 2, 5, 0, 1, 2, 5]), vec![1,2,5], vec![3,4]);
        assert_eq!(Some(Ok(0)), r.next());
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(0)), r.next());
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(None, r.next());

        let mut r = replace(generate_okok_iterator(vec![0, 1, 2, 5, 0, 1, 2, 5]), vec![0,1,2], vec![3,4]);
        
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(5)), r.next());
        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(5)), r.next());
        assert_eq!(None, r.next());
    }


    #[test]
    fn test1_error() {
        let mut r = replace(generate_okokerr_iterator(vec![1,1,1,1,1], error::overflow_error("[test]".to_string())), vec![1,1,1], vec![3,4]);
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());
        // println!("{:?}", r.next());

        assert_eq!(Some(Ok(3)), r.next());
        assert_eq!(Some(Ok(4)), r.next());
        assert_eq!(Some(Ok(1)), r.next());
        assert_eq!(Some(Ok(1)), r.next());
        assert_eq!(error::Kind::OverflowError, r.next().unwrap().err().unwrap().kind());
        assert_eq!(None, r.next());
    }
}