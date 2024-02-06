/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#spy
pub fn spy<I>(iterable: I, n: usize) -> Option<Vec<I::Item>> 
where
    I: IntoIterator,
{
    let mut iter = iterable.into_iter();
    let mut ret = Vec::new();
    for _ in 0..n {
        match iter.next() {
            None => { break; }
            Some(v) => { ret.push(v) }
        }
    }
    return Some(ret);
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        match spy(v, 1) {
            Some(ret) => { assert_eq!(ret, vec![1]); },
            None => { assert!(false); }
        }

        let v = vec![1,2,3,4,5];
        match spy(v, 0) {
            Some(ret) => { assert_eq!(ret, vec![]); },
            None => { assert!(false); }
        }

        let v = vec![1,2,3,4,5];
        match spy(v, 3) {
            Some(ret) => { assert_eq!(ret, vec![1,2,3]); },
            None => { assert!(false); }
        }

        let v = vec![1,2,3,4,5];
        match spy(v, 7) {
            Some(ret) => { assert_eq!(ret, vec![1,2,3,4,5]); },
            None => { assert!(false); }
        }
    }
}