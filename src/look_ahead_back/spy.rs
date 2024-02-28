/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#spy
pub fn spy<T>(iter: &mut Box<dyn Iterator<Item = T>>, n: usize) -> Option<Vec<T>> 
where
T: 'static
{
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
    
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        assert_eq!(vec![1], spy(&mut iter_from_vec(v), 1).unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(Vec::<i32>::new(), spy(&mut iter_from_vec(v), 0).unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(vec![1,2,3], spy(&mut iter_from_vec(v), 3).unwrap());

        let v = vec![1,2,3,4,5];
        assert_eq!(vec![1,2,3,4,5], spy(&mut iter_from_vec(v), 7).unwrap());
    }
}