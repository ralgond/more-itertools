use crate::sequence::{create_seq_from_iterator, Sequence};

pub struct CountCycle<T> 
where
T: Clone
{
    seq: Box<dyn Sequence<T>>,
    n: usize,
    cur_num: usize,
    cur_iter_idx: usize
}

impl<T> Iterator for CountCycle<T>
where 
T: Clone
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.seq.len() == 0 {
            return None;
        }

        if self.cur_num == self.n {
            return None;
        }

        if self.cur_iter_idx == self.seq.len() {
            self.cur_num += 1;
            self.cur_iter_idx = 0;
        }

        if self.cur_num == self.n {
            return None;
        }

        let ret = (self.cur_num, self.seq.get(self.cur_iter_idx).unwrap().clone());
        self.cur_iter_idx += 1;

        return Some(ret);
    }
}

pub fn count_cycle<T: 'static>(iter: Box<dyn Iterator<Item=T>>, n: usize) -> Box<dyn Iterator<Item=(usize,T)>> 
where
T: Clone
{
    let cc = CountCycle {
        seq: Box::new(create_seq_from_iterator(iter)),
        n: n,
        cur_num: 0,
        cur_iter_idx: 0
    };

    return Box::new(cc);
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec!['A', 'B'];
        let mut cc = count_cycle(iter_from_vec(v), 3);

        assert_eq!(Some((0, 'A')), cc.next());
        assert_eq!(Some((0, 'B')), cc.next());
        assert_eq!(Some((1, 'A')), cc.next());
        assert_eq!(Some((1, 'B')), cc.next());
        assert_eq!(Some((2, 'A')), cc.next());
        assert_eq!(Some((2, 'B')), cc.next());
        assert_eq!(None, cc.next());
        assert_eq!(None, cc.next());


        let v = Vec::<char>::new();
        let mut cc = count_cycle(iter_from_vec(v), 3);
        assert_eq!(None, cc.next());
    }
}