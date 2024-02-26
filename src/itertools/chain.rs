
pub struct Chain<T> {
    input: Vec<Box<dyn Iterator<Item=T>>>,
    cur_idx: usize,
    iter_finished: bool
}

impl<T> Iterator for Chain<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                return None;
            }
    
            if self.cur_idx >= self.input.len() {
                self.iter_finished = true;
                return None;
            }
    
            let cur = self.input.get_mut(self.cur_idx).unwrap();
            let _next = cur.next();
            match _next {
                None => {
                    self.cur_idx += 1;
                    continue;
                },
                Some(v) => {
                    return Some(v);
                }
            }
        }
    }
}

pub fn chain<T: 'static>(input: Vec<Box<dyn Iterator<Item=T>>>) -> Box<dyn Iterator<Item=T>>  {
    Box::new(Chain {
        input,
        cur_idx: 0,
        iter_finished: false
    })
}

#[cfg(test)]
mod tests {
    use crate::itertools::map::map;

    use super::*;

    #[test]
    fn test1() {
        let mut input = Vec::new();

        let v = vec![1,2,3];
        let ret1 = map(Box::new(v.clone().into_iter()), |x| {x});
        input.push(ret1);

        let ret2 = map(Box::new(v.clone().into_iter()), |x| {x*2});
        input.push(ret2);

        let chain = chain(input);

        assert_eq!(vec![1,2,3,2,4,6], chain.collect::<Vec<_>>());
    }
}