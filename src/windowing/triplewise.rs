use std::fmt::Debug;

use super::sliding_window::sliding_windowed;
use crate::error::Error;

pub fn triplewise<T>(iter: Box<dyn Iterator<Item=T>>) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where
T: Clone + Debug + 'static
{
    return sliding_windowed(iter, 3);
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4];
        let mut pw = triplewise(iter_from_vec(v));

        assert_eq!(pw.next().unwrap().ok().unwrap(), vec![0, 1, 2]);
        assert_eq!(pw.next().unwrap().ok().unwrap(), vec![1, 2, 3]);
        assert_eq!(pw.next().unwrap().ok().unwrap(), vec![2, 3, 4]);
        assert_eq!(pw.next(), None);
    }
}