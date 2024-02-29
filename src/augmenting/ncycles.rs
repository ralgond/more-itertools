use crate::sequence::Sequence;


pub struct Ncycles<T> {
    seq: Box<dyn Sequence<T>>,
    n: usize,
    cur: usize,
    loop_cnt: usize
}

impl <T> Iterator for Ncycles<T> 
where
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seq.len() == 0 || self.cur >= self.seq.len() {
            return None;
        }

        if self.loop_cnt >= self.n {
            return None;
        }

        let ret = self.seq.get(self.cur).unwrap().clone();

        self.cur += 1;

        if self.cur >= self.seq.len() {
            self.cur = 0;
            self.loop_cnt += 1;
        }

        return Some(ret);
    }
}

pub fn ncycles<T>(seq: Box<dyn Sequence<T>>, n: usize) -> Box<dyn Iterator<Item = T>> 
where T: Clone + 'static
{
    return Box::new(Ncycles {
        seq,
        n,
        cur:0,
        loop_cnt: 0
    });
}

#[cfg(test)]
mod tests {
    use crate::sequence::create_seq_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec!['A', 'B'];
        let nc = ncycles(create_seq_from_vec(v), 3);
        assert_eq!(vec!['A', 'B', 'A', 'B', 'A', 'B'], nc.collect::<Vec<_>>());

        let v = Vec::<char>::new();
        let nc = ncycles(create_seq_from_vec(v), 3);
        assert_eq!(Vec::<char>::new(), nc.collect::<Vec<_>>());
    }
}