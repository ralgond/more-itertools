use std::collections::VecDeque;

#[allow(dead_code)]
pub struct TeeInner<T> {
    buf1: VecDeque<T>,
    buf2: VecDeque<T>,
    offset1: usize,
    offset2: usize
}


