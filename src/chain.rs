use std::iter;

pub fn chain<I, J>(
    i: I,
    j: J,
) -> iter::Chain<<I as IntoIterator>::IntoIter, <J as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator<Item = I::Item>,
{
    i.into_iter().chain(j)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = chain(vec![1,2,3], [4,5,6]);
        assert_eq!(Some(1), c.next());
        assert_eq!(Some(2), c.next());
        assert_eq!(Some(3), c.next());
        assert_eq!(Some(4), c.next());
        assert_eq!(Some(5), c.next());
        assert_eq!(Some(6), c.next());
        assert_eq!(None, c.next());
    }
}