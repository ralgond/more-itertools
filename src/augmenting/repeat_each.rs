use std::collections::VecDeque;


pub struct RepeatEach<T> 
where
T: Clone
{
    iter: Box<dyn Iterator<Item = T>>,
    n: usize,
    item_buffer: VecDeque<T>,
    emit_count: usize,
    iter_finished: bool,
    emit_finished : bool
}


impl<T> Iterator for RepeatEach<T>
where
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.emit_finished {
                return None;
            }

            if !self.iter_finished && self.item_buffer.len() == 0 {
                let _next = self.iter.next();
                match _next {
                    None => {
                        self.iter_finished = true;
                        self.emit_finished = true;
                        return None;
                    },
                    Some(v) => {
                        self.item_buffer.push_back(v);
                        //assert!(self.item_buffer.len() == 1)
                    }
                }
            }

            if self.emit_count == self.n {
                self.item_buffer.pop_front();
                self.emit_count = 0;
                continue;
            } else {
                self.emit_count += 1;
                return Some(self.item_buffer.front_mut().unwrap().clone());
            }
        }
    }
}

pub fn repeat_each<T>(iter: Box<dyn Iterator<Item=T>>, n: usize) -> Box<dyn Iterator<Item=T>>
where
T: Clone + 'static
{
    Box::new(RepeatEach {
        iter,
        n,
        item_buffer: VecDeque::new(),
        emit_count: 0,
        iter_finished: false,
        emit_finished: false
    })
}

#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let ret = repeat_each(iter_from_vec(v), 3).collect::<Vec<_>>();
        assert_eq!(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], ret);

        let v = vec![1,2,3];
        let ret = repeat_each(iter_from_vec(v), 0).collect::<Vec<_>>();
        assert_eq!(0, ret.len());
    }
}