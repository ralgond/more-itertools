
pub trait Sequence<T> {
    fn get<'a>(&'a self, index: usize) -> Option<&'a T>;
    fn len(&self) -> usize;
}

pub struct SequenceVector<T> {
    v: Vec<T>
}

impl<T> SequenceVector<T> {
    pub(crate) fn new(v: Vec<T>) -> SequenceVector<T> {
        SequenceVector {
            v
        }
    }
}

impl<T> Sequence<T> for SequenceVector<T> {
    fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        return self.v.get(index);
    }

    fn len(&self) -> usize {
        return self.v.len();
    }
}

pub fn create_seq_from_vec<T>(v: Vec<T>) -> impl Sequence<T> {
    return SequenceVector::new(v);
}

#[cfg(test)]
mod tests {
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
}