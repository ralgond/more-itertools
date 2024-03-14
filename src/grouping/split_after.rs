use crate::{error::Error, others::cache_until::cache_until};

pub fn splite_after<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, 
    pred: fn(&T) -> Result<bool,Error>,
    maxsplit: i128
) -> Box<dyn Iterator<Item = Result<Vec<T>, Error>>>
where 
T: Clone + 'static
{
    cache_until(iter, pred, maxsplit, true)
}


#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_after(generate_okok_iterator(v), |x|{Ok(x%3==0)}, -1);
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![1,2,3])), r.next());
        assert_eq!(Some(Ok(vec![4,5,6])), r.next());
        assert_eq!(Some(Ok(vec![7,8,9])), r.next());
        assert_eq!(Some(Ok(Vec::<i32>::new())), r.next());
        assert_eq!(None, r.next());
        assert_eq!(None, r.next());

        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_after(generate_okok_iterator(v), |x|{Ok(x%3==0)}, 2);
        assert_eq!(Some(Ok(vec![0])), r.next());
        assert_eq!(Some(Ok(vec![1,2,3])), r.next());
        assert_eq!(Some(Ok(vec![4,5,6,7,8,9])), r.next());

        let v = vec![0,1,2,3,4,5,6,7,8,9];
        let mut r = splite_after(generate_okok_iterator(v), |x|{Ok(x%3==0)}, 0);
        assert_eq!(Some(Ok(vec![0,1,2,3,4,5,6,7,8,9])), r.next());
        assert_eq!(None, r.next());
    }
}