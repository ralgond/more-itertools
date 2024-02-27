struct Zip<T0, T1> {
    iter0: Box<dyn Iterator<Item=T0>>,
    iter1: Box<dyn Iterator<Item=T1>>,
    iter_finished: bool
}

impl<T0,T1> Iterator for Zip<T0,T1>
{
    type Item = (T0,T1);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        let _next0 = self.iter0.next();
        let _next1 = self.iter1.next();
        match (_next0, _next1) {
            (Some(v0), Some(v1)) => {
                return Some((v0, v1));
            },
            _=> { 
                self.iter_finished = true;
                return None;
            }
        }
    }
}

pub fn zip<T0: 'static,T1: 'static>(iter0: Box<dyn Iterator<Item=T0>>, iter1: Box<dyn Iterator<Item=T1>>) -> Box<dyn Iterator<Item=(T0,T1)>> {
    Box::new(Zip{iter0,iter1,iter_finished:false})
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let ret = zip(iter_from_vec(vec![1,2,3]), iter_from_vec(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let v = vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())];
        assert_eq!(v, ret.collect::<Vec<_>>());


        let ret = zip(iter_from_vec(vec![1,2,3,4]), iter_from_vec(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let v = vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())];
        assert_eq!(v, ret.collect::<Vec<_>>());

        let ret = zip(iter_from_vec(vec![1,2,3]), iter_from_vec(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]));
        let v = vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())];
        assert_eq!(v, ret.collect::<Vec<_>>());
    }
}
