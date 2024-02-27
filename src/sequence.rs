

pub trait Sequence<T>
{
    fn get<'a>(&'a self, index: usize) -> Option<&'a T>;
    fn len(&self) -> usize;
    fn slice(&self, begin: usize, end: usize) -> &[T];
    fn as_slice(&self) -> &[T];
}

pub struct SequenceVector<T> {
    v: Vec<T>
}

impl<T> SequenceVector<T> 
where T: 'static
{
    pub(crate) fn new(v: Vec<T>) -> Box<dyn Sequence<T>> {
        Box::new(SequenceVector {
            v
        })
    }
}

impl<T> Sequence<T> for SequenceVector<T>
{
    fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        return self.v.get(index);
    }

    fn len(&self) -> usize {
        return self.v.len();
    }

    fn slice(&self, begin: usize, end: usize) -> &[T] {
        return &self.v[begin..end];
    }

    fn as_slice(&self) -> &[T] {
        return self.v.as_slice();
    }
}

pub fn are_seqs_equals<T>(seq1: &dyn Sequence<T>, seq2: &dyn Sequence<T>) -> bool
where T: PartialEq
{
    return seq1.len() == seq2.len() && seq1.as_slice().starts_with(seq2.as_slice());
}

pub fn create_seq_from_vec<T>(v: Vec<T>) -> Box<dyn Sequence<T>> 
where T: 'static
{
    return SequenceVector::new(v);
}

pub fn create_seq_from_iterator<T>(mut iter: Box<dyn Iterator<Item=T>>) -> Box<dyn Sequence<T>>
where T: 'static
{
    let mut v = Vec::new();
    loop {
        let _next = iter.next();
        match _next {
            None => { break; }
            Some(_v) => {
                v.push(_v);
            }
        }
    }

    return SequenceVector::new(v);
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let v = create_seq_from_vec(v);
        assert_eq!(3, v.len());
        assert_eq!(1, *v.get(0).unwrap());
        assert_eq!(2, *v.get(1).unwrap());
        assert_eq!(3, *v.get(2).unwrap());
    }

    #[test]
    fn test2() {
        let v = iter_from_vec(vec![1,2,3]);
        let v = create_seq_from_iterator(v);
        assert_eq!(3, v.len());
        assert_eq!(1, *v.get(0).unwrap());
        assert_eq!(2, *v.get(1).unwrap());
        assert_eq!(3, *v.get(2).unwrap());
    }
}