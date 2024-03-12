use crate::error::Error;

use super::interleave_longest::interleave_longest;


pub fn roundrobin<T>(iter_vec: Vec<Box<dyn Iterator<Item = Result<T,Error>>>>) -> Box<dyn Iterator<Item = Result<T,Error>>> 
where
T: Clone + 'static
{
    return interleave_longest(iter_vec, None);
}

#[cfg(test)]
mod tests {

    use crate::utils::{extract_value_from_result_vec, generate_okok_iterator};

    use super::*;

    #[test]
    fn test1() {
        let mut v = Vec::new();
        v.push(generate_okok_iterator("ABC".chars().collect::<Vec<_>>()));
        v.push(generate_okok_iterator("D".chars().collect::<Vec<_>>()));
        v.push(generate_okok_iterator("EF".chars().collect::<Vec<_>>()));

        let ret = roundrobin(v).collect::<Vec<_>>();
        assert_eq!(vec!['A', 'D', 'E', 'B', 'F', 'C'], extract_value_from_result_vec(ret).0);
    }
}