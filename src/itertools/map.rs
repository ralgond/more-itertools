
struct Map<T, J> {
    iter: Box<dyn Iterator<Item=T>>,
    pred: fn(T)->J,
    iter_finished: bool
}

impl<T,J> Iterator for Map<T, J>
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
                let j = (self.pred)(v);
                return Some(j);
            }
        }
    }
}

pub fn map<T: 'static, J: 'static>(iter: Box<dyn Iterator<Item=T>>, pred: fn(T)->J) -> Box<dyn Iterator<Item=J>> 
{
    return Box::new(Map {
        iter,
        pred: pred,
        iter_finished: false
    });
}


struct Map2<T0, T1, J> {
    iter0: Box<dyn Iterator<Item=T0>>,
    iter1: Box<dyn Iterator<Item=T1>>,
    pred: fn(T0, T1)->J,
    iter_finished: bool
}

impl<T0, T1, J> Iterator for Map2<T0, T1, J>
{
    type Item = J;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_finished {
            return None;
        }
        let _next0 = self.iter0.next();
        let _next1 = self.iter1.next();
        match (_next0, _next1) {
            (Some(v0), Some(v1)) => {
                let j = (self.pred)(v0, v1);
                return Some(j);
            },
            _ => {
                self.iter_finished = true;
                return None;
            },
        }
    }
}

pub fn map2<T0: 'static, T1: 'static, J: 'static>(
    iter0: Box<dyn Iterator<Item=T0>>, 
    iter1: Box<dyn Iterator<Item=T1>>,
    pred: fn(T0,T1)->J) -> Box<dyn Iterator<Item=J>> 
{
    return Box::new(Map2 {
        iter0,
        iter1,
        pred: pred,
        iter_finished: false
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let ret = map(Box::new(v.into_iter()), |x| {x==3});
        assert_eq!(vec![false,false,true], ret.collect::<Vec<_>>());
    }

    #[test]
    fn test2() {
        let v0: Vec<i32> = vec![1,2,3];
        let v1: Vec<i32> = vec![2,3,4];
        let ret = map2(
            Box::new(v0.into_iter()), 
            Box::new(v1.into_iter()), |x,y| {x*y});
        assert_eq!(vec![2,6,12], ret.collect::<Vec<_>>());
    }
}

