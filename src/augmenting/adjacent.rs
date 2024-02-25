use std::iter::Zip;

use crate::itertools::tee::TeeCursor;
use crate::itertools::{tee::tee, map::map};
use crate::windowing::windowed::windowed;
use crate::utils::any_result;

use crate::itertools::chain::chain; 

fn false_padding(n: usize) -> Vec<bool> { 
    let mut padding = Vec::<bool>::new();
    for _ in 0..n {
        padding.push(false);
    }
    return padding;
}

pub fn adjacent<I>(i: I, pred: fn(&I::Item)->bool, distance: usize) -> Zip<std::vec::IntoIter<bool>, TeeCursor<<I as IntoIterator>::IntoIter>>
where
    I: IntoIterator,
    I::Item: Clone
{
    let (i1, i2) = tee(i.into_iter());
    let left_padding = false_padding(distance);
    let right_padding = false_padding(distance);

    let selected = chain!(left_padding, map(i1, pred), right_padding);
    

    let adjacent_to_selected = map(windowed(selected, 2 * distance + 1, 1), any_result).collect::<Vec<_>>();
    // println!("adjacent_to_selected={:?}", adjacent_to_selected);

    let ret0: std::iter::Zip<std::vec::IntoIter<bool>, crate::itertools::tee::TeeCursor<<I as IntoIterator>::IntoIter>> = adjacent_to_selected.into_iter().zip(i2.into_iter());
    
    return ret0;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let adj = adjacent(v, |x| {*x == 3}, 1);
        assert_eq!(vec![(false, 1), (true, 2), (true, 3), (true, 4), (false, 5)], adj.collect::<Vec<_>>());

        let v = vec![1,2,3,4,5];
        let adj = adjacent(v, |x| {*x == 1 || *x == 5}, 1);
        assert_eq!(vec![(true, 1), (true, 2), (false, 3), (true, 4), (true, 5)], adj.collect::<Vec<_>>());

        let v = vec![1,2,3,4,5];
        let adj = adjacent(v, |x| {*x == 1 || *x == 5}, 2);
        assert_eq!(vec![(true, 1), (true, 2), (true, 3), (true, 4), (true, 5)], adj.collect::<Vec<_>>());

        let v = vec![1,2,3,4,5];
        let adj = adjacent(v, |x| {*x == 1 || *x == 5}, 0);
        assert_eq!(vec![(true, 1), (false, 2), (false, 3), (false, 4), (true, 5)], adj.collect::<Vec<_>>());
    }
}