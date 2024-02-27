use crate::itertools::islice::islice;

pub fn nth<T>(iter: Box<dyn Iterator<Item = T>>, n: usize, default: Option<T>) -> Option<T>
where
T: 'static
{
    let mut i = islice(iter, n, n+1, 1);

    match i.next() {
        None => {
            match default {
                Some(_) => { return default; }
                None => { return None; }
            }
        },
        Some(v) => {
            match v {
                Ok(ok_v) => { return Some(ok_v); }
                Err(_) => { return None; }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Some(5), nth(iter_from_vec(vec![0,1,2,3,4,5]), 5, Some(1)));
        assert_eq!(Some(0), nth(iter_from_vec(vec![0,1,2,3,4,5]), 7, Some(0)));
        assert_eq!(None, nth(iter_from_vec(vec![0,1,2,3,4,5]), 7, None));
    }
}