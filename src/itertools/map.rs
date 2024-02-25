pub struct Map<I, J>
where
I: Iterator
{
    iter: I,
    pred: fn(&I::Item) -> J,
    iter_finished: bool,
}

impl<I,J> Iterator for Map<I,J> 
where 
    I: Iterator
{
    type Item = J;

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
                return Some((self.pred)(&v));
            }
        }
    }
}

pub fn map<I,J>(i: I, pred: fn(&I::Item)->J) -> Map<I::IntoIter, J>
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