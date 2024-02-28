pub struct BeforeAndAfter<T>
where
T: Clone
{
    before: Vec<T>,
    the_one: Option<T>,
    consumed_the_one: bool,
    after: Option<Box<dyn Iterator<Item=T>>>,
}

impl<T> Iterator for BeforeAndAfter<T>    
where
T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.consumed_the_one {
            self.consumed_the_one = true;
            match &self.the_one {
                None => { return None; },
                Some(v) => { return Some(v.clone()); }
            }
            //return None;
        } else {
            match &mut self.after {
                None => { return None; }
                Some(v) => {
                    return v.next();
                }
            }
        }
    }
}


pub fn before_and_after<T> (iterable: Box<dyn Iterator<Item=T>>, predicate: fn(item: &T)->bool) -> BeforeAndAfter<T>
where
    T: Clone,
{
    let mut ret = BeforeAndAfter {
        before: Vec::new(),
        after: None,
        the_one: None,
        consumed_the_one: false
    };

    let mut into_iter = iterable.into_iter();

    loop {
        match into_iter.next() {
            None => {break;}
            Some(ret_val) => {
                if predicate(&ret_val) {
                    ret.before.push(ret_val);
                } else {
                    ret.the_one = Some(ret_val);
                    ret.after = Some(into_iter);
                    break;
                }
            }
        }
    } 

    return ret;
}

#[cfg(test)]
mod tests {

    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v1 = String::from("ABCdEfGhI");
        let baa = before_and_after(iter_from_vec(v1.chars().collect()), |x: &char| { x.is_ascii_uppercase() });

        assert_eq! (vec!['A', 'B', 'C'], baa.before);

        let v = baa.collect::<Vec<_>>();
        assert_eq!(vec!['d', 'E', 'f', 'G', 'h', 'I'], v);
    }

}