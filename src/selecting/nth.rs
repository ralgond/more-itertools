use crate::islice::islice;

pub fn nth<I>(iterable: I, n: usize, default: Option<I::Item>) -> Option<I::Item>
where
    I: IntoIterator
{
    let mut i = islice(iterable, n, n+1, 1);

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
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Some(5), nth(vec![0,1,2,3,4,5], 5, Some(1)));
        assert_eq!(Some(0), nth(vec![0,1,2,3,4,5], 7, Some(0)));
    }
}