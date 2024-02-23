use std::collections::VecDeque;


pub struct RepeatEach<I: Iterator> 
where
I::Item: Clone
{
    iter: I,
    n: usize,
    item_buffer: VecDeque<I::Item>,
    emit_count: usize,
    iter_finished: bool,
    emit_finished : bool
}


impl<I: Iterator> Iterator for RepeatEach<I>
where
I::Item: Clone
{
    type Item = <I as Iterator>::Item;

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

pub fn repeat_each<I>(iterable: I, n: usize) -> RepeatEach<I::IntoIter>
where
I: IntoIterator,
I::Item: Clone
{
    RepeatEach {
        iter: iterable.into_iter(),
        n: n,
        item_buffer: VecDeque::new(),
        emit_count: 0,
        iter_finished: false,
        emit_finished: false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let ret = repeat_each(v, 3).collect::<Vec<_>>();
        assert_eq!(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], ret);

        let v = vec![1,2,3];
        let ret = repeat_each(v, 0).collect::<Vec<_>>();
        assert_eq!(0, ret.len());
    }
}