
pub struct Padded<I: Iterator> 
where
I::Item: Clone
{
    iter: I,
    fill_value: I::Item,
    n: usize,
    next_multiple: bool,
    offset: usize,
    iter_finished: bool,
    __n: usize
}


impl<I: Iterator> Iterator for Padded<I>
where
I::Item: Clone
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_multiple {
            if !self.iter_finished {
                let _next = self.iter.next();
                match _next {
                    None => {
                        self.iter_finished = true;
                        for i in 1.. {
                            self.__n = self.n * i;
                            if self.__n >= self.offset {
                                break;
                            }
                        }
                    },
                    Some(v) => {
                        self.offset += 1;
                        return Some(v);
                    }
                }
            }

            if self.__n <= self.offset {
                return None;
            } else {
                self.offset += 1;
                return Some(self.fill_value.clone());
            }
        } else {
            if !self.iter_finished {
                let _next = self.iter.next();
                match _next {
                    None => {
                        self.iter_finished = true;
                    },
                    Some(v) => {
                        self.offset += 1;
                        return Some(v);
                    }
                }
            }

            if self.n <= self.offset {
                return None;
            } else {
                self.offset += 1;
                return Some(self.fill_value.clone());
            }
        }
    }
}

pub fn padded<I>(iterable: I, fill_value: I::Item, n: usize, next_multiple: bool) -> Padded<I::IntoIter>
where
I: IntoIterator,
I::Item: Clone
{
    Padded {
        iter: iterable.into_iter(),
        fill_value: fill_value,
        n: n,
        next_multiple: next_multiple,
        offset: 0,
        iter_finished: false,
        __n: 0
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let p = padded(v, 0, 5, false);
        assert_eq!(vec![1,2,3,0,0], p.collect::<Vec<_>>());

        let v = vec![1,2,3];
        let p = padded(v, 0, 2, false);
        assert_eq!(vec![1,2,3], p.collect::<Vec<_>>());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3];
        let p = padded(v, 0, 5, true);
        assert_eq!(vec![1,2,3,0,0], p.collect::<Vec<_>>());

        let v = vec![1,2,3];
        let p = padded(v, 0, 2, true);
        assert_eq!(vec![1,2,3,0], p.collect::<Vec<_>>());

        let v = vec![1,2,3,4];
        let p = padded(v, 0, 2, true);
        assert_eq!(vec![1,2,3,4], p.collect::<Vec<_>>());

        let v = vec![1,2,3,4];
        let p = padded(v, 0, 4, true);
        assert_eq!(vec![1,2,3,4], p.collect::<Vec<_>>());
    }
}