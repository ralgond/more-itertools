#[derive(Debug, Clone)]
pub struct Iterate<Item> {
    func: fn(item: &Item) -> Item,
    start: Item
}


impl<Item> Iterator for Iterate<Item> 
where
Item: Clone
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.start.clone();
        self.start = (self.func)(&self.start);
        return Some(ret);
    }
}


pub fn iterate<Item>(func: fn(item: &Item) -> Item, start: Item) -> Iterate<Item>
{
    Iterate {
        func: func,
        start: start
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut it = iterate(|x| { x * 2 }, 1);
        assert_eq!(Some(1), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(8), it.next());
        assert_eq!(Some(16), it.next());
        assert_eq!(Some(32), it.next());
    }
}

