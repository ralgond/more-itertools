pub struct RepeatLast<I: Iterator> 
where
I::Item: Clone
{
    iter: I,
    default_item: Option<I::Item>,
    last_item: Option<I::Item>,
    iter_finished: bool,
}


impl<I: Iterator> Iterator for RepeatLast<I>
where
I::Item: Clone
{
    type Item = <I as Iterator>::Item;

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

pub fn repeat_last<I>(iterable: I, default_item: I::Item) -> RepeatLast<I::IntoIter>
where
I: IntoIterator,
I::Item: Clone
{
    RepeatLast {
        iter: iterable.into_iter(),
        default_item: Some(default_item),
        last_item: None,
        iter_finished: false
    }
}

#[cfg(test)]
mod tests {

    use crate::{itertools::islice::islice, utils::extract_value_from_result_vec};

    use super::*;

    #[test]
    fn test1() {

        let rl = repeat_last(vec![1,2,3], 0);
        assert_eq!((vec![1, 2, 3, 3, 3], false), extract_value_from_result_vec(islice(rl, 0, 5, 1).collect::<Vec<_>>()));

        let rl = repeat_last(Vec::<i32>::new(), 42);
        assert_eq!((vec![42,42,42,42,42], false), extract_value_from_result_vec(islice(rl, 0, 5, 1).collect::<Vec<_>>()));
    }
}