
#[derive(Debug,Clone)]
pub struct Iterate<T> {
    func: fn(&T) -> T,
    start: T
}

impl<T> Iterator for Iterate<T> 
where 
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.start.clone();
        self.start = (self.func)(&self.start);
        return Some(ret);
    }
}


pub fn iterate<T>(func: fn(&T) -> T, start: T) -> Box<dyn Iterator<Item = T>>
where
T: Clone + 'static
{
    Box::new(Iterate {
        func: func,
        start: start
    })
}

#[cfg(test)]
mod tests {
    use crate::{itertools::islice::islice, utils::extract_value_from_result_vec};

    use super::*;

    #[test]
    fn test1() {
        let mut it = iterate(|x| { x * 2 }, 1).into_iter();
        assert_eq!(Some(1), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(8), it.next());
        assert_eq!(Some(16), it.next());
    }

    #[test]
    fn test2() {
        let i = islice(iterate(|x| { x * 2 }, 1), 0, 10, 1);
        let j = extract_value_from_result_vec(i.collect::<Vec<_>>());
        // println!("{:?}", j.0);
        assert_eq!(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512], j.0);
    }
}