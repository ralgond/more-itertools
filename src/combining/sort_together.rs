use crate::utils::argsort;

pub fn sort_together2<T1, T2>(v1: &Vec<T1>, v2: &Vec<T2>, reverse: bool) -> (Vec<T1>, Vec<T2>)
where T1: Ord + Clone,
T2: Clone
{
    let mut sort_result = argsort(v1.as_slice());
    if reverse {
        sort_result.reverse();
    }
    let mut ret1 = Vec::<T1>::with_capacity(v1.len());
    let mut ret2 = Vec::<T2>::with_capacity(v1.len());
    for old_index in sort_result.into_iter() {
        ret1.push(v1.get(old_index).unwrap().clone());
        ret2.push(v2.get(old_index).unwrap().clone());
    }

    return (ret1, ret2);
}

pub fn sort_together3<T1, T2, T3>(v1: &Vec<T1>, v2: &Vec<T2>, v3: &Vec<T3>, reverse: bool) -> (Vec<T1>, Vec<T2>, Vec<T3>)
where T1: Ord + Clone,
T2: Clone,
T3: Clone
{
    let mut sort_result = argsort(v1.as_slice());
    if reverse {
        sort_result.reverse();
    }
    let mut ret1 = Vec::<T1>::with_capacity(v1.len());
    let mut ret2 = Vec::<T2>::with_capacity(v1.len());
    let mut ret3 = Vec::<T3>::with_capacity(v1.len());
    for old_index in sort_result.into_iter() {
        ret1.push(v1.get(old_index).unwrap().clone());
        ret2.push(v2.get(old_index).unwrap().clone());
        ret3.push(v3.get(old_index).unwrap().clone());
    }

    return (ret1, ret2, ret3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ret = sort_together2(&vec![4, 3, 2, 1], &vec!['a', 'b', 'c', 'd'], false);
        assert_eq!((vec![1, 2, 3, 4], vec!['d', 'c', 'b', 'a']), ret);

        let ret = sort_together2(&vec![4, 3, 2, 1], &vec!['a', 'b', 'c', 'd'], true);
        assert_eq!((vec![4, 3, 2, 1], vec!['a', 'b', 'c', 'd']), ret);
    }

    #[test]
    fn test2() {
        let ret = sort_together3(&vec![4, 3, 2, 1], &vec!['a', 'b', 'c', 'd'], 
                                                &vec![5, 6, 7, 8], false);
        assert_eq!((vec![1, 2, 3, 4], vec!['d', 'c', 'b', 'a'], vec![8, 7, 6, 5]), ret);

        let ret = sort_together3(&vec![4, 3, 2, 1], &vec!['a', 'b', 'c', 'd'],
                                                                &vec![8, 7, 6, 5], true);
        assert_eq!((vec![4, 3, 2, 1], vec!['a', 'b', 'c', 'd'], vec![8, 7, 6, 5]), ret);
    }
}