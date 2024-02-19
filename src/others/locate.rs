#[derive(Debug,Clone)]
pub struct Locate<T> {
    array: Vec<T>,
    query: Vec<T>,
    offset: usize,
    window_size: usize
}

impl<T> Iterator for Locate<T> 
where 
T: Clone + PartialEq
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.window_size != self.query.len() {
            return None;
        }

        loop {
            if self.array.len() - self.offset < self.window_size {
                return None;
            }
    
            let slice = &self.array[self.offset..self.offset+self.window_size];

            if slice.len() == self.query.len() && self.query.starts_with(slice) {
                let ret_offset = self.offset;
    
                self.offset += 1;
        
                return Some(ret_offset);
            } else {
                self.offset += 1;
            }
        }
    }
}

pub fn locate<T>(
                    array: Vec<T>,
                    query: Vec<T>,
                    window_size: usize) -> Locate<T>
{
    Locate {
        array: array,
        query: query,
        offset: 0,
        window_size: window_size
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l = locate(vec![1,1,1,1,1], vec![1,1,1], 3);
        assert_eq!(Some(0), l.next());
        assert_eq!(Some(1), l.next());
        assert_eq!(Some(2), l.next());
        assert_eq!(None, l.next());
        assert_eq!(None, l.next());

        let mut l = locate(vec![1,1,1,1,1], vec![1], 1);
        assert_eq!(Some(0), l.next());
        assert_eq!(Some(1), l.next());
        assert_eq!(Some(2), l.next());
        assert_eq!(Some(3), l.next());
        assert_eq!(Some(4), l.next());
        assert_eq!(None, l.next());
    }
}