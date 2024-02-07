
pub fn take<I>(it: I, n: usize) -> Vec<I::Item> 
where
    I: IntoIterator
{
    let mut iter = it.into_iter();

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

    use super::*;

    #[test]
    fn test1() {
        let v1 = vec![1,2,3,4,5];

        assert_eq!(vec![1,2,3], take(v1, 3));

        let v2 = vec![1,2,3,4,5];

        assert_eq!(vec![1,2,3,4,5], take(v2, 10));
    }
}