use crate::error::Error;


pub fn iter_from_vec<T: 'static>(v: Vec<T>) -> Box<dyn Iterator<Item=T>> {
    return Box::new(v.into_iter());
}

pub fn iter_from_result_vec<T: 'static>(v: Vec<Result<T,Error>>) -> Box<dyn Iterator<Item=Result<T,Error>>> {
    return Box::new(v.into_iter());
}