struct Filter<T> {
    iter: Box<dyn Iterator<Item = T>>,
    iter_finished: bool,
    pred: fn(&T) -> bool
}

impl<T> Iterator for Filter<T> 
where T: 'static
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }

        loop {
            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    return None;
                },
                Some(v) => {
                    if (self.pred)(&v) {
                        return Some(v);
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}

pub fn filter<T>(iter: Box<dyn Iterator<Item = T>>, pred: fn(&T) -> bool) -> Box<dyn Iterator<Item = T>> 
where T: 'static
{
    return Box::new(Filter{
        iter,
        iter_finished: false,
        pred
    })
}

#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::filter;


    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6,7,8,9];
        let ret = filter(iter_from_vec(v), |x| {x % 2 == 0}).collect::<Vec<_>>();
        assert_eq!(vec![2,4,6,8], ret);
    }
}