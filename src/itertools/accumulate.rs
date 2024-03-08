use crate::error::Error;
use crate::error;

pub struct Accumulate 
{
    cur_sum: usize,
    cur_idx: usize,
    v: Vec<usize>
}

impl Iterator for Accumulate
{
    type Item = Result<usize, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx >= self.v.len() {
            return None;
        }

        let ret = usize::overflowing_add(self.cur_sum, self.v[self.cur_idx]);
        if ret.1 {
            // overflow
            return Some(Err(error::any_error(error::Kind::OverflowError, "Add overflow.".to_string())));
        } else {
            self.cur_idx += 1;
            self.cur_sum = ret.0;
            return Some(Ok(ret.0));
        }
    }
}

pub fn accumulate(v: Vec<usize>) -> Accumulate {
    return Accumulate {
        cur_sum: 0,
        cur_idx: 0,
        v: v
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::extract_value_from_result_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![4,3,3];

        let a = accumulate(v);

        assert_eq!((vec![4, 7, 10], None), extract_value_from_result_vec(a.collect::<Vec<_>>()));
    }

}