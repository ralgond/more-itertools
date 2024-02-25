#[macro_export]
macro_rules! chain {
    () => {
        core::iter::empty()
    };
    ($first:expr $(, $rest:expr )* $(,)?) => {
        {
            let iter = core::iter::IntoIterator::into_iter($first);
            $(
                let iter =
                    core::iter::Iterator::chain(
                        iter,
                        core::iter::IntoIterator::into_iter($rest));
            )*
            iter
        }
    };
}

pub(crate) use chain;

#[cfg(test)]
mod tests {
    use crate::itertools::map::map;

    use super::*;

    #[test]
    fn test1() {
        let mut c = chain!(vec![1,2,3], [4,5,6]);
        assert_eq!(Some(1), c.next());
        assert_eq!(Some(2), c.next());
        assert_eq!(Some(3), c.next());
        assert_eq!(Some(4), c.next());
        assert_eq!(Some(5), c.next());
        assert_eq!(Some(6), c.next());
        assert_eq!(None, c.next());
    }

    #[test]
    fn test2() {
        let c = chain!(vec![1,2,3], vec![4,5,6], vec![7,8,9]);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9], c.collect::<Vec<_>>());

        let c = chain!(Vec::new(), vec![4,5,6], vec![7,8,9]);
        assert_eq!(vec![4,5,6,7,8,9], c.collect::<Vec<_>>());

        let c = chain!(Vec::new(), Vec::new(), vec![7,8,9]);
        assert_eq!(vec![7,8,9], c.collect::<Vec<_>>());

        let c = chain!(Vec::<i32>::new(), Vec::new(), Vec::new());
        assert_eq!(Vec::<i32>::new(), c.collect::<Vec<_>>());
    }

    #[test]
    fn test3() {
        let c = chain!(vec![1,2,3], map(vec![4,5,6], |x|{x*2}));
        assert_eq!(vec![1,2,3,8,10,12], c.collect::<Vec<_>>());

        let c = chain!(vec![1,2,3], map(vec![4,5,6], |x|{x*2}), vec![1,2,3]);
        assert_eq!(vec![1,2,3,8,10,12,1,2,3], c.collect::<Vec<_>>());
    }
}