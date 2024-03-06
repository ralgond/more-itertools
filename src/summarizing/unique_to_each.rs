use std::collections::HashSet;

use crate::{error, error::Error, sequence::{create_seq_from_iterator_result, Sequence}};


pub fn unique_to_each<T>(iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>) -> Result<Vec<Box<dyn Sequence<T>>>, Error> 
where T: 'static
{
    let mut seq_vec: Vec<Box<dyn Sequence<T>>> = Vec::new();
    let mut result: Vec<Box<dyn Sequence<T>>> = Vec::new();
    
    for iter in iter_vec.into_iter() {
        let seq_result = create_seq_from_iterator_result(iter)?;
        seq_vec.push(seq_result);
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use crate::utils::{generate_okok_iterator, generate_okokerr_iterator};

    use super::*;

    #[test]
    fn test1() {
        let v1 = generate_okok_iterator(vec![1,2,3]);
        let v2 = generate_okokerr_iterator(vec![1,2,3], error::overflow_error("overflow".to_string()));
        
        let mut v = Vec::new();
        v.push(v1);
        v.push(v2);
        
        let result = unique_to_each(v);
        if let Err(err) = result {
            assert_eq!(error::Kind::OverflowError, err.kind());
        }
    }
}

