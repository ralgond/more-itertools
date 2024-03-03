
pub fn take<T>(iter: &mut Box<dyn Iterator<Item = T>>, n: usize) -> Vec<T> 
where
T: 'static
{
    let mut ret = Vec::new();

    for _ in 0..n {
        let item = iter.next();
        match item {
            None => { break; }
            Some(v) => {
                ret.push(v);
            }
        }
    }

    return ret;
}

#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v1 = vec![1,2,3,4,5];

        let mut iter1 = iter_from_vec(v1);

        assert_eq!(vec![1,2,3], take(&mut iter1, 3));

        assert_eq!(vec![4,5], iter1.collect::<Vec<_>>());

        let v2 = vec![1,2,3,4,5];

        assert_eq!(vec![1,2,3,4,5], take(&mut iter_from_vec(v2), 10));
    }
}