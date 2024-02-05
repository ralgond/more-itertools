

use std::str::Chars;

pub struct SubstringsIndexes<'a> {
    string_len: usize,
    iter: Chars<'a>,
    substring_len: usize,
    cur: usize,
    vec: Vec<char>,
    first_iter_loop_finished: bool,
    reverse: bool,
}

impl<'a> Iterator for SubstringsIndexes<'a>
{
    type Item = (String, usize, usize);

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
                    for ele in self.vec[self.cur..self.cur+self.substring_len].iter() {
                        ret.push(ele.clone().to_string())
                    }
                    let cur = self.cur;
                    self.cur += 1;

                    let ret2 = ret.join("");
                    return Some((ret2, cur, cur+self.substring_len));
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
                    for ele in self.vec[self.cur..self.cur+self.substring_len].iter() {
                        ret.push(ele.clone().to_string())
                    }
                    let cur = self.cur;
                    self.cur -= 1;

                    let ret2 = ret.join("");
                    return Some((ret2, cur, cur+self.substring_len));
                }
            }
        }
    }
}


pub fn substrings_indexes<'a>(s: &'a String, reverse: bool) -> SubstringsIndexes<'a> {
    
    let mut ret = SubstringsIndexes {
        string_len: s.len(),
        iter: s.chars(),
        substring_len: 1,
        cur: 0,
        vec: Vec::new(),
        first_iter_loop_finished: false,
        reverse: reverse
    };

    if ret.reverse {
        ret.substring_len = s.len();
        ret.cur = s.len() - ret.substring_len;
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = "more".to_string();
        let mut ssi = substrings_indexes(&v, false);

        assert_eq!(Some(("m".to_string(), 0 as usize, 1 as usize)), ssi.next());
        assert_eq!(Some(("o".to_string(), 1 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some(("r".to_string(), 2 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some(("e".to_string(), 3 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("mo".to_string(), 0 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some(("or".to_string(), 1 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some(("re".to_string(), 2 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("mor".to_string(), 0 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some(("ore".to_string(), 1 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("more".to_string(), 0 as usize, 4 as usize)), ssi.next());
        assert_eq!(None, ssi.next());
    }

    #[test]
    fn test2() {
        let v = "more".to_string();
        let mut ssi = substrings_indexes(&v, true);

        
        assert_eq!(Some(("more".to_string(), 0 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("ore".to_string(), 1 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("mor".to_string(), 0 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some(("re".to_string(), 2 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("or".to_string(), 1 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some(("mo".to_string(), 0 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some(("e".to_string(), 3 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some(("r".to_string(), 2 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some(("o".to_string(), 1 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some(("m".to_string(), 0 as usize, 1 as usize)), ssi.next());
        assert_eq!(None, ssi.next());
    }
}