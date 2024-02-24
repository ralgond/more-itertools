pub struct Map<I>
where
I: Iterator
{
    iter: I,
    pred: fn(&I::Item) -> bool,
    iter_finished: bool,
}

impl<I> Iterator for Map<I> 
where 
    I: Iterator
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        let _next = self.iter.next();
        match _next {
            None => {
                self.iter_finished = true;
                return None;
            },
            Some(v) => {
                if (self.pred)(&v) {
                    return Some(true);
                } else {
                    return Some(false);
                }
            }
        }
    }
}

pub fn map<I>(i: I, pred: fn(&I::Item)->bool) -> Map<I::IntoIter>
where
    I: IntoIterator
{
    Map {
        iter: i.into_iter(),
        pred: pred,
        iter_finished: false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let m = map(v, |x| {*x == 3});
        assert_eq!(vec![false,false,true], m.collect::<Vec<_>>());
    }
}