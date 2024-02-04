use super::windowed::{windowed, Windowed};

pub fn sliding_windowed<I>(iterable: I, n: usize) -> Windowed<I::IntoIter>
where
    I: IntoIterator,
    I::Item: Clone
{
    return windowed(iterable, n, 1);
}


mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4,5];

        let mut w = sliding_windowed(v, 4);
        match w.next().unwrap() {
            Ok(v) => { assert_eq!(vec![0,1,2,3], v); }
            Err(_) => { assert!(false); }
        }
        match w.next().unwrap() {
            Ok(v) => { assert_eq!(vec![1,2,3,4], v); }
            Err(_) => { assert!(false); }
        }
        match w.next().unwrap() {
            Ok(v) => { assert_eq!(vec![2,3,4,5], v); }
            Err(_) => { assert!(false); }
        }
        match w.next() {
            Some(_) => { assert!(false); }
            None => { assert!(true); }
        }
    }

    #[test]
    fn test2() {
        let v = vec![0,1,2];
        let mut w = sliding_windowed(v, 4);
        match w.next() {
            Some(_) => { assert!(false); }
            None => { assert!(true); }
        }
    }
}
