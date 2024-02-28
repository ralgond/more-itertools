use crate::sequence::Sequence;

pub struct SubstringsIndexes<T> {
    string_len: usize,
    substring_len: usize,
    cur: usize,
    vec: Box<dyn Sequence<T>>,
    //first_iter_loop_finished: bool,
    reverse: bool,
}

impl<T> Iterator for SubstringsIndexes<T>
where
T: Clone
{
    type Item = (Vec<T>, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
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
                    for ele in self.vec.slice(self.cur, self.cur+self.substring_len) {
                        ret.push(ele.clone())
                    }
                    let cur = self.cur;
                    self.cur += 1;

                    return Some((ret, cur, cur+self.substring_len));
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
                    for ele in self.vec.slice(self.cur, self.cur+self.substring_len) {
                        ret.push(ele.clone())
                    }
                    let cur = self.cur;
                    self.cur -= 1;

                    return Some((ret, cur, cur+self.substring_len));
                }
            }
        }
    }
}


pub fn substrings_indexes<T>(seq: Box<dyn Sequence<T>>, reverse: bool) -> Box<dyn Iterator<Item = (Vec<T>,usize,usize)>> 
where
T: Clone + 'static
{
    
    let seq_len = seq.len();

    let mut ret = SubstringsIndexes {
        string_len: seq_len,
        substring_len: 1,
        cur: 0,
        vec: seq,
        //first_iter_loop_finished: false,
        reverse: reverse
    };

    if ret.reverse {
        ret.substring_len = seq_len;
        ret.cur = seq_len - ret.substring_len;
    }

    return Box::new(ret);
}

#[cfg(test)]
mod tests {
    use crate::sequence::create_seq_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = create_seq_from_vec("more".to_string().chars().collect());
        let mut ssi = substrings_indexes(v, false);

        assert_eq!(Some((vec!['m'], 0 as usize, 1 as usize)), ssi.next());
        assert_eq!(Some((vec!['o'], 1 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some((vec!['r'], 2 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some((vec!['e'], 3 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['m', 'o'], 0 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some((vec!['o', 'r'], 1 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some((vec!['r', 'e'], 2 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['m', 'o', 'r'], 0 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some((vec!['o', 'r', 'e'], 1 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['m', 'o', 'r', 'e'], 0 as usize, 4 as usize)), ssi.next());
        assert_eq!(None, ssi.next());
    }

    #[test]
    fn test2() {
        let v = create_seq_from_vec("more".to_string().chars().collect());
        let mut ssi = substrings_indexes(v, true);

        assert_eq!(Some((vec!['m', 'o', 'r', 'e'], 0 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['o', 'r', 'e'], 1 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['m', 'o', 'r'], 0 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some((vec!['r', 'e'], 2 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['o', 'r'], 1 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some((vec!['m', 'o'], 0 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some((vec!['e'], 3 as usize, 4 as usize)), ssi.next());
        assert_eq!(Some((vec!['r'], 2 as usize, 3 as usize)), ssi.next());
        assert_eq!(Some((vec!['o'], 1 as usize, 2 as usize)), ssi.next());
        assert_eq!(Some((vec!['m'], 0 as usize, 1 as usize)), ssi.next());
        assert_eq!(None, ssi.next());
    }
}