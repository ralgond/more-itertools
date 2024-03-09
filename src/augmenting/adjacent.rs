use crate::error::Error;
use crate::itertools::{tee::tee, map::map, zip::zip};
use crate::windowing::windowed::windowed;
use crate::utils::{any1, generate_okok_iterator};

use crate::itertools::chain::chain; 

fn false_padding(n: usize) -> Vec<bool> { 
    let mut padding = Vec::<bool>::new();
    for _ in 0..n {
        padding.push(false);
    }
    return padding;
}

pub fn adjacent<T>(i: Box<dyn Iterator<Item=Result<T,Error>>>, 
                    pred: fn(T)->Result<bool,Error>, distance: usize) -> Box<dyn Iterator<Item = Result<(bool, T), Error>>>
where
T: Clone + 'static
{
    let (i1, i2) = tee(i);
    let left_padding = generate_okok_iterator(false_padding(distance));
    let right_padding = generate_okok_iterator(false_padding(distance));
    let input  = vec![left_padding, map(i1, pred), right_padding];

    let selected = chain(input);
    

    let adjacent_to_selected = map(windowed(selected, 2 * distance + 1, 1), any1);
    // println!("adjacent_to_selected={:?}", adjacent_to_selected);

    let ret0 = zip(adjacent_to_selected, i2);
    
    return ret0;
}


#[cfg(test)]
mod tests {
    use crate::utils::extract_value_from_result_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let adj = adjacent(generate_okok_iterator(v), |x| {Ok(x == 3)}, 1);
        assert_eq!(vec![(false, 1), (true, 2), (true, 3), (true, 4), (false, 5)], extract_value_from_result_vec(adj.collect::<Vec<_>>()).0);

        let v = vec![1,2,3,4,5];
        let adj = adjacent(generate_okok_iterator(v), |x| {Ok(x == 1 || x == 5)}, 1);
        assert_eq!(vec![(true, 1), (true, 2), (false, 3), (true, 4), (true, 5)], extract_value_from_result_vec(adj.collect::<Vec<_>>()).0);

        let v = vec![1,2,3,4,5];
        let adj = adjacent(generate_okok_iterator(v), |x| {Ok(x == 1 || x == 5)}, 2);
        assert_eq!(vec![(true, 1), (true, 2), (true, 3), (true, 4), (true, 5)], extract_value_from_result_vec(adj.collect::<Vec<_>>()).0);

        let v = vec![1,2,3,4,5];
        let adj = adjacent(generate_okok_iterator(v), |x| {Ok(x == 1 || x == 5)}, 0);
        assert_eq!(vec![(true, 1), (false, 2), (false, 3), (false, 4), (true, 5)], extract_value_from_result_vec(adj.collect::<Vec<_>>()).0);
    }
}