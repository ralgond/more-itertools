use super::interleave_longest::interleave_longest;


pub fn roundrobin<T>(iter_vec: Vec<Box<dyn Iterator<Item = T>>>) -> Box<dyn Iterator<Item = T>> 
where
T: Clone + 'static
{
    return interleave_longest(iter_vec, None);
}

#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let mut v = Vec::new();
        v.push(iter_from_vec("ABC".chars().collect::<Vec<_>>()));
        v.push(iter_from_vec("D".chars().collect::<Vec<_>>()));
        v.push(iter_from_vec("EF".chars().collect::<Vec<_>>()));

        let ret = roundrobin(v).collect::<Vec<_>>();
        assert_eq!(vec!['A', 'D', 'E', 'B', 'F', 'C'], ret);
    }
}