use crate::error::Error;
use crate::error;

#[derive(Debug,Clone)]
pub struct Rlocate<T> {
    array: Vec<T>,
    query: Vec<T>,
    offset: usize,
    offset_overflow: bool,
}

impl<T> Iterator for Rlocate<T> 
where 
T: PartialEq
{
    type Item = Result<usize, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset_overflow {
            return Some(Err(error::overflow_error("offset overflow.".to_string())));
        }

        loop {
            if self.offset == usize::MAX {
                return None;
            }
    
            let slice = &self.array[self.offset..self.offset+self.query.len()];

            if slice.len() == self.query.len() && self.query.starts_with(slice) {
                let ret_offset = self.offset;
    
                self.offset -= 1;
        
                return Some(Ok(ret_offset));
            } else {
                self.offset -= 1;
            }
        }
    }
}

pub fn rlocate<T>(
                    array: Vec<T>,
                    query: Vec<T>) -> Rlocate<T>
{
    let array_len_sub_query_len = array.len().overflowing_sub(query.len());
    Rlocate {
        array: array,
        query: query,
        offset: array_len_sub_query_len.0,
        offset_overflow: array_len_sub_query_len.1,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l = rlocate(vec![1,1,1,1,1], vec![1,1,1]);
        assert_eq!(Some(Ok(2)), l.next());
        assert_eq!(Some(Ok(1)), l.next());
        assert_eq!(Some(Ok(0)), l.next());
        assert_eq!(None, l.next());
        assert_eq!(None, l.next());

        let mut l = rlocate(vec![1,1,1,1,1], vec![1]);
        assert_eq!(Some(Ok(4)), l.next());
        assert_eq!(Some(Ok(3)), l.next());
        assert_eq!(Some(Ok(2)), l.next());
        assert_eq!(Some(Ok(1)), l.next());
        assert_eq!(Some(Ok(0)), l.next());
        assert_eq!(None, l.next());
        assert_eq!(None, l.next());

        let mut l = rlocate(vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3], vec![1,2,3]);
        assert_eq!(Some(Ok(9)), l.next());
        assert_eq!(Some(Ok(5)), l.next());
        assert_eq!(Some(Ok(1)), l.next());
        assert_eq!(None, l.next());

        let mut l = rlocate(vec![0, 1], vec![1,2,3]);
        let ret = l.next();
        if let Some(v) = ret {
            match v {
                Err(_) => { assert!(true); }
                Ok(_) => { assert!(false); }
            }
        }
    }
}