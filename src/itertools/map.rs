
struct Map<T, J> {
    input: Box<dyn Iterator<Item=T>>,
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
        let _next = self.input.next();
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

pub fn map<T: 'static, J: 'static>(i: Box<dyn Iterator<Item=T>>, pred: fn(T)->J) -> Box<dyn Iterator<Item=J>> 
{
    return Box::new(Map {
        input: i,
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
}

