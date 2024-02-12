use crate::error;

use super::distribute::{distribute, Distribute};

struct ChunkedEven<T> {
    dist: Distribute<T>,
    cur: usize
}

pub fn chunked_even<T>(buf: Vec<T>, bucket_cnt: usize) -> ChunkedEven<T> 
where
T: Clone + 'static
{
    return ChunkedEven {
        dist: distribute(buf, bucket_cnt),
        cur: 0
    };
}

impl<T> Iterator for ChunkedEven<T>
where
T: Clone + 'static
{
    type Item = Result<Vec<T>, error::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.dist.bucket_count {
            let mut cursor = self.dist.iter(self.cur);
            self.cur += 1;

            let mut ret = Vec::new();
            loop {
                let v = cursor.next();
                match v {
                    None => { break; }
                    Some(v2) => {
                        match v2 {
                            Ok(v3) => { ret.push(v3); },
                            Err(e) => { return Some(Err(e))}
                        }
                    }
                }
            }

            return Some(Ok(ret));
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9,10];
        let mut ce = chunked_even(v, 3);

        assert_eq!(Some(Ok(vec![1, 4, 7, 10])), ce.next());
        assert_eq!(Some(Ok(vec![2, 5, 8])), ce.next());
        assert_eq!(Some(Ok(vec![3, 6, 9])), ce.next());
        assert_eq!(None, ce.next());
        assert_eq!(None, ce.next());
        assert_eq!(None, ce.next());
    }
}