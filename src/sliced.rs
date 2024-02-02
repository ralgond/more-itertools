
// use crate::error::Error;
// use crate::error;

// #[derive(Debug, Clone)]
// #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
// pub struct Sliced<'a, T> {
//     seq: &'a [T],
//     n: usize,
//     strict: bool
// }

// impl<'a, T> Iterator for Sliced<'a, T> {
//     type Item = Result<Vec<&'a T>, Error>;

//     fn next(&mut self) -> Option<Self::Item> {
//         return Some(Ok(&self.seq[0..3]));
//     }
// }

// pub fn sliced<'a, T>(seq: &'a [T], n: usize, strict: bool) -> Sliced<'a, T> {
//     Sliced {
//         seq: seq,
//         n: n,
//         strict: strict
//     }
// }