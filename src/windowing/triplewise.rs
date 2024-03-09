use std::fmt::Debug;

use super::sliding_window::sliding_windowed;
use crate::error::Error;

pub fn triplewise<T>(iter: Box<dyn Iterator<Item=Result<T,Error>>>) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where
T: Clone + Debug + 'static
{
    return sliding_windowed(iter, 3);
}

#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4];
        let mut pw = triplewise(generate_okok_iterator(v));

        assert_eq!(pw.next().unwrap().ok().unwrap(), vec![0, 1, 2]);
        assert_eq!(pw.next().unwrap().ok().unwrap(), vec![1, 2, 3]);
        assert_eq!(pw.next().unwrap().ok().unwrap(), vec![2, 3, 4]);
        assert_eq!(pw.next(), None);
    }
}