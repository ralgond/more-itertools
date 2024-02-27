use std::collections::VecDeque;

struct MarkEndsOutputItem<T> {
    sentinel: bool,
    item: Option<T>
}

pub struct MarkEnds<T> 
{
    iter: Box<dyn Iterator<Item=T>>,
    emitted_head: bool,
    buffer: VecDeque<MarkEndsOutputItem<T>>,
    iter_finished: bool
}


impl<T> Iterator for MarkEnds<T>
{
    type Item = (bool, bool, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.buffer.len() == 1 {
                if self.buffer.front().unwrap().sentinel {
                    self.buffer.pop_front();
                    return None;
                }
            } else if self.buffer.len() == 2 {
                if self.buffer.back().unwrap().sentinel {
                    let ret = self.buffer.pop_front().unwrap();
                    self.buffer.pop_front(); // pop sentinel
                    if self.emitted_head {
                        return Some((false, true, ret.item.unwrap()));
                    } else {
                        self.emitted_head = true;
                        return Some((true, true, ret.item.unwrap()));
                    }
                } else {
                    // tow (false,false)
                    let ret = self.buffer.pop_front().unwrap();
                    if self.emitted_head {
                        return Some((false, false, ret.item.unwrap()));
                    } else {
                        self.emitted_head = true;
                        return Some((true, false, ret.item.unwrap()));
                    }
                }
            }

            if self.iter_finished {
                return None;
            }
    
            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    self.buffer.push_back(MarkEndsOutputItem{
                        sentinel: true,
                        item: None
                    });
                } Some(v) => {
                    self.buffer.push_back(MarkEndsOutputItem{
                        sentinel: false,
                        item: Some(v)
                    });
                }
            }
        }
    }
}

pub fn mark_ends<T>(iter: Box<dyn Iterator<Item=T>>) -> Box<dyn Iterator<Item=(bool,bool,T)>>
where T: 'static
{
    Box::new(MarkEnds {
        iter: iter,
        buffer: VecDeque::new(),
        emitted_head: false,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let me = mark_ends(iter_from_vec(v));
        assert_eq!(vec![(true, false, 1), (false, false, 2), (false, true, 3)], me.collect::<Vec<_>>());

        let v = vec![1,2];
        let me = mark_ends(iter_from_vec(v));
        assert_eq!(vec![(true, false, 1), (false, true, 2)], me.collect::<Vec<_>>());

        let v = vec![1];
        let me = mark_ends(iter_from_vec(v));
        assert_eq!(vec![(true, true, 1)], me.collect::<Vec<_>>());

        let v = Vec::<(bool, bool, i32)>::new();
        let me = mark_ends(iter_from_vec(v));
        // println!("{:?}", me.collect::<Vec<_>>());
        assert_eq!(0, me.collect::<Vec<_>>().len());
    }
}