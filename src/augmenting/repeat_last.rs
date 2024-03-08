pub struct RepeatLast<T> 
where
T: Clone
{
    iter: Box<dyn Iterator<Item = T>>,
    default_item: Option<T>,
    last_item: Option<T>,
    iter_finished: bool,
}


impl<T> Iterator for RepeatLast<T>
where
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                match &self.last_item {
                    None => {
                        return Some(self.default_item.as_mut().unwrap().clone());
                    },
                    Some(v) => {
                        return Some(v.clone());
                    }
                }
            }

            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                },
                Some(v) => {
                    self.last_item = Some(v);
                    return Some(self.last_item.as_mut().unwrap().clone());
                }
            }
        }
    }
}

pub fn repeat_last<T>(iter: Box<dyn Iterator<Item=T>>, default_item: T) -> Box<dyn Iterator<Item=T>>
where
T: Clone + 'static
{
    Box::new(RepeatLast {
        iter,
        default_item: Some(default_item),
        last_item: None,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {

    use crate::{itertools::{islice::islice, iter::iter_from_vec}, utils::extract_value_from_result_vec};

    use super::*;

    #[test]
    fn test1() {

        let rl = repeat_last(iter_from_vec(vec![1,2,3]), 0);
        assert_eq!((vec![1, 2, 3, 3, 3], None), extract_value_from_result_vec(islice(rl, 0, 5, 1).collect::<Vec<_>>()));

        let rl = repeat_last(iter_from_vec(Vec::<i32>::new()), 42);
        assert_eq!((vec![42,42,42,42,42], None), extract_value_from_result_vec(islice(rl, 0, 5, 1).collect::<Vec<_>>()));
    }
}