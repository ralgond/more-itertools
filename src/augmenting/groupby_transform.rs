use std::collections::VecDeque;
use std::fmt::Debug;
use crate::utils::are_same;

pub struct GroupByTransform<T, TKey, TValue, TReduce> 
where T: Clone + Debug + PartialEq,
{
    key_buf: VecDeque<TKey>,
    funced_cur_key: Option<TKey>,
    value_buf: Vec<TValue>,
    input_buf: VecDeque<T>,
    iter: Box<dyn Iterator<Item = T>>,
    iter_finished: bool,


    keyfunc: fn(&T)->TKey,
    valuefunc: fn(&T)->TValue,
    reducefunc: fn(&Vec<TValue>)-> TReduce
}


impl<T, TKey, TValue, TReduce> Iterator for GroupByTransform<T, TKey, TValue, TReduce> 
where
T: PartialEq + Clone + Debug,
TKey: PartialEq + Clone + Debug
{
    type Item = (TKey, TReduce);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished && self.value_buf.len() == 0 {
                return None;
            }

            if self.value_buf.len() > 0 {
                let funced_cur_key = self.key_buf.pop_front();
                let reduce_result = (self.reducefunc)(&self.value_buf);
                self.value_buf = Vec::new();
                return Some((funced_cur_key.as_ref().unwrap().clone(), reduce_result))
            }
        
            loop {
                let _next = self.iter.next();
                match _next {
                    None => {
                        self.iter_finished = true;
                    },
                    Some(v) => {
                        self.input_buf.push_back(v);
                    }
                }

                if self.input_buf.len() == 0 {
                    break;
                }

                let v = self.input_buf.front().unwrap().clone();

                let funced_key = (self.keyfunc)(&v);

                let funced_value = (self.valuefunc)(&v);
                
                if !self.funced_cur_key.is_none() {
                    if are_same(Some(self.funced_cur_key.as_ref().unwrap()), Some(&funced_key)) {
                        self.value_buf.push(funced_value);
                        self.input_buf.pop_front();
                    } else {
                        // funced_cur_key != funced_key
                        self.funced_cur_key = Some(funced_key);
                        self.key_buf.push_back(self.funced_cur_key.as_ref().unwrap().clone());
                        break;
                    }
                } else {
                    self.funced_cur_key = Some((self.keyfunc)(&v));
                    self.key_buf.push_back(self.funced_cur_key.as_ref().unwrap().clone());
                    println!("====>{:?}", self.funced_cur_key);
                    break;
                }
            }
        }
    }
}

pub fn groupby_transform<T, TKey, TValue, TReduce>(iter: Box<dyn Iterator<Item = T>>,
    keyfunc: fn(&T)->TKey,
    valuefunc: fn(&T)->TValue,
    reducefunc: fn(&Vec<TValue>)-> TReduce
    ) -> GroupByTransform<T, TKey, TValue, TReduce>
where
T: PartialEq + Clone + Debug
{
    return GroupByTransform {
        key_buf: VecDeque::new(),
        funced_cur_key: None,
        value_buf: Vec::new(),
        input_buf: VecDeque::new(),
        iter,
        iter_finished: false,

        keyfunc,
        valuefunc,
        reducefunc,
    }
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut ret: GroupByTransform<char, char, char, String> = groupby_transform(iter_from_vec("aAAAbbbcccC".chars().collect::<Vec<char>>()),
            |x| {x.to_ascii_uppercase()},
            |x| { *x },
            |x| { return String::from_iter(x); }
        );

        assert_eq!(Some(('A', "aAAA".to_string())), ret.next());
        assert_eq!(Some(('B', "bbb".to_string())), ret.next());
        assert_eq!(Some(('C', "cccC".to_string())), ret.next());
        assert_eq!(None, ret.next());
        assert_eq!(None, ret.next());
    }

    #[test]
    fn test2() {
        let mut ret: GroupByTransform<char, char, char, String> = groupby_transform(iter_from_vec("aAAAbbbcccC".chars().collect::<Vec<char>>()),
            |x| {x.to_ascii_uppercase()},
            |x| { x.to_ascii_lowercase() },
            |x| { return String::from_iter(x); }
        );

        assert_eq!(Some(('A', "aaaa".to_string())), ret.next());
        assert_eq!(Some(('B', "bbb".to_string())), ret.next());
        assert_eq!(Some(('C', "cccc".to_string())), ret.next());
        assert_eq!(None, ret.next());
        assert_eq!(None, ret.next());
    }
}