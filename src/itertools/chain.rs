use std::{collections::VecDeque, iter};

pub fn chain<I, J>(
    i: I,
    j: J,
) -> iter::Chain<<I as IntoIterator>::IntoIter, <J as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator<Item = I::Item>,
{
    i.into_iter().chain(j)
}

pub struct Chain3<I> 
{
    iters: VecDeque<I>,
    iter_finished: bool
}

impl<I: Iterator> Iterator for Chain3<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }

            if self.iters.len() == 0 {
                self.iter_finished = true;
                continue;
            }

            let _next = self.iters.front_mut().unwrap().next();
            match _next {
                None => {
                    self.iters.pop_front();
                    continue;
                },
                Some(v) => {
                    return Some(v);
                }
            }
        }
    }
}

pub fn chain3<I>(i: I, j: I, k: I) -> Chain3<I::IntoIter>
where
    I: IntoIterator
{
    let mut ret = Chain3 {
        iters: VecDeque::new(),
        iter_finished: false
    };

    ret.iters.push_back(i.into_iter());
    ret.iters.push_back(j.into_iter());
    ret.iters.push_back(k.into_iter());

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = chain(vec![1,2,3], [4,5,6]);
        assert_eq!(Some(1), c.next());
        assert_eq!(Some(2), c.next());
        assert_eq!(Some(3), c.next());
        assert_eq!(Some(4), c.next());
        assert_eq!(Some(5), c.next());
        assert_eq!(Some(6), c.next());
        assert_eq!(None, c.next());
    }

    #[test]
    fn test2() {
        let c = chain3(vec![1,2,3], vec![4,5,6], vec![7,8,9]);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9], c.collect::<Vec<_>>());

        let c = chain3(Vec::new(), vec![4,5,6], vec![7,8,9]);
        assert_eq!(vec![4,5,6,7,8,9], c.collect::<Vec<_>>());

        let c = chain3(Vec::new(), Vec::new(), vec![7,8,9]);
        assert_eq!(vec![7,8,9], c.collect::<Vec<_>>());

        let c = chain3(Vec::<i32>::new(), Vec::new(), Vec::new());
        assert_eq!(Vec::<i32>::new(), c.collect::<Vec<_>>());
    }
}