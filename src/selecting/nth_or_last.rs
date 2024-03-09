use crate::error::Error;
use crate::others::cache_last::cache_last;

/// https://more-itertools.readthedocs.io/en/v10.2.0/_modules/more_itertools/more.html#nth_or_last
pub fn nth_or_last<T>(iter: Box<dyn Iterator<Item = Result<T,Error>>>, n: usize, default: Option<T>) -> Option<Result<T, Error>>
where
T: Clone + 'static
{
    let mut cl = cache_last(iter);
    let mut cl_iter = cl.iter_with_emit_first();

    let mut i = 0;
    while i < n {
        let _next = cl_iter.next();
        if let Some(v_next) = _next {
            match v_next {
                Ok(_) => {
                    i += 1;
                    continue;
                },
                Err(err_v_next) => { // upstream error
                    return Some(Err(err_v_next));
                }
            }
        } else {
            break;
        }
    }

    if i < n {
        cl.insert_last_to_head(); 
    }
    
    if cl.is_empty() {
        if let Some(v_default) = default {
            return Some(Ok(v_default));
        } else {
            return None;
        }
    } else {
        return cl.get_last_item();
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::generate_okok_iterator;

    use super::*;

    #[test]
    fn test1() {
        let ret = nth_or_last(generate_okok_iterator(vec![0,1,2,3]), 3, Some(5));
        assert_eq!(2, ret.unwrap().ok().unwrap());

        let ret2 = nth_or_last(generate_okok_iterator(vec![0,1]), 2, Some(5));
        assert_eq!(1, ret2.unwrap().ok().unwrap());

        let ret3 = nth_or_last(generate_okok_iterator(vec![]), 0, Some(5));
        assert_eq!(5, ret3.unwrap().ok().unwrap());

        let ret4 = nth_or_last(generate_okok_iterator(vec![]), 0, None::<i32>);
        assert_eq!(None, ret4);
    }
}