use crate::error::Error;
use crate::error;

#[derive(Debug,Clone)]
pub struct Locate<T> {
    array: Vec<T>,
    query: Vec<T>,
    offset: usize
}

impl<T> Iterator for Locate<T> 
where 
T: PartialEq
{
    type Item = Result<usize, Error>;

    fn next(&mut self) -> Option<Self::Item> {

        loop {
            let of_add = self.query.len().overflowing_add(self.offset);
            if of_add.1 {
                //overflow
                return Some(Err(error::overflow_error("offset overflow".to_string())));
            }

            if self.array.len() < self.query.len() + self.offset {
                return None;
            }
    
            let slice = &self.array[self.offset..self.offset+self.query.len()];

            if slice.len() == self.query.len() && self.query.starts_with(slice) {
                let ret_offset = self.offset;
    
                self.offset += 1;
        
                return Some(Ok(ret_offset));
            } else {
                self.offset += 1;
            }
        }
    }
}

pub fn locate<T>(
                    array: Vec<T>,
                    query: Vec<T>) -> Locate<T>
{
    Locate {
        array: array,
        query: query,
        offset: 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l = locate(vec![1,1,1,1,1], vec![1,1,1]);
        assert_eq!(Some(Ok(0)), l.next());
        assert_eq!(Some(Ok(1)), l.next());
        assert_eq!(Some(Ok(2)), l.next());
        assert_eq!(None, l.next());
        assert_eq!(None, l.next());

        let mut l = locate(vec![1,1,1,1,1], vec![1]);
        assert_eq!(Some(Ok(0)), l.next());
        assert_eq!(Some(Ok(1)), l.next());
        assert_eq!(Some(Ok(2)), l.next());
        assert_eq!(Some(Ok(3)), l.next());
        assert_eq!(Some(Ok(4)), l.next());
        assert_eq!(None, l.next());


        let mut l = locate(vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3], vec![1,2,3]);
        assert_eq!(Some(Ok(1)), l.next());
        assert_eq!(Some(Ok(5)), l.next());
        assert_eq!(Some(Ok(9)), l.next());
        assert_eq!(None, l.next());
    }
}