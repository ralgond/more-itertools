
struct Prepend<T> {
    value: T,
    emitted_value: bool,
    iter: Box<dyn Iterator<Item = T>>,
    iter_finished: bool
}

impl<T> Iterator for Prepend<T> 
where T: Clone + 'static
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        if !self.emitted_value {
            self.emitted_value = true;
            return Some(self.value.clone());
        }

        let _next = self.iter.next();
        match _next {
            None => {
                self.iter_finished = true;
                return None;
            },
            Some(v) => {
                return Some(v);
            }
        }
    }
}

pub fn prepend<T>(value: T, iter: Box<dyn Iterator<Item = T>>) -> Box<dyn Iterator<Item = T>> 
where
T: Clone + 'static
{
    Box::new(Prepend {
        value,
        emitted_value: false,
        iter,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![0,1,2,3], prepend(0, iter_from_vec(vec![1,2,3])).collect::<Vec<_>>());
    }
}