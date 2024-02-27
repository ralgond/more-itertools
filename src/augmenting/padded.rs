
pub struct Padded<T> 
where
T: Clone
{
    iter: Box<dyn Iterator<Item=T>>,
    fill_value: T,
    n: usize,
    next_multiple: bool,
    offset: usize,
    iter_finished: bool,
    __n: usize
}


impl<T> Iterator for Padded<T>
where
T: Clone
{
    type Item = T;

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

pub fn padded<T>(iter: Box<dyn Iterator<Item=T>>, fill_value: T, n: usize, next_multiple: bool) -> Box<dyn Iterator<Item=T>>
where
T: Clone + 'static
{
    Box::new(Padded {
        iter,
        fill_value,
        n,
        next_multiple,
        offset: 0,
        iter_finished: false,
        __n: 0
    })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let p = padded(iter_from_vec(v), 0, 5, false);
        assert_eq!(vec![1,2,3,0,0], p.collect::<Vec<_>>());

        let v = vec![1,2,3];
        let p = padded(iter_from_vec(v), 0, 2, false);
        assert_eq!(vec![1,2,3], p.collect::<Vec<_>>());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3];
        let p = padded(iter_from_vec(v), 0, 5, true);
        assert_eq!(vec![1,2,3,0,0], p.collect::<Vec<_>>());

        let v = vec![1,2,3];
        let p = padded(iter_from_vec(v), 0, 2, true);
        assert_eq!(vec![1,2,3,0], p.collect::<Vec<_>>());

        let v = vec![1,2,3,4];
        let p = padded(iter_from_vec(v), 0, 2, true);
        assert_eq!(vec![1,2,3,4], p.collect::<Vec<_>>());

        let v = vec![1,2,3,4];
        let p = padded(iter_from_vec(v), 0, 4, true);
        assert_eq!(vec![1,2,3,4], p.collect::<Vec<_>>());
    }
}