
pub fn iter_from_vec<T: 'static>(v: Vec<T>) -> Box<dyn Iterator<Item=T>> {
    return Box::new(v.into_iter());
}