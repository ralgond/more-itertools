

pub fn unzip2<T0, T1>(iter: &mut Box<dyn Iterator<Item = (T0,T1)>>) -> (Vec<T0>, Vec<T1>) {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();

    loop {
        let ret = iter.next();
        match ret {
            None => { break; },
            Some(v) => {
                t0.push(v.0);
                t1.push(v.1);
            }
        }
    }
    return (t0, t1);
}

pub fn unzip3<T0, T1, T2>(iter: &mut Box<dyn Iterator<Item = (T0,T1,T2)>>) -> (Vec<T0>, Vec<T1>, Vec<T2>) {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();

    loop {
        let ret = iter.next();
        match ret {
            None => { break; },
            Some(v) => {
                t0.push(v.0);
                t1.push(v.1);
                t2.push(v.2);
            }
        }
    }
    return (t0, t1, t2);
}

pub fn unzip4<T0, T1, T2, T3>(iter: &mut Box<dyn Iterator<Item = (T0,T1,T2,T3)>>) -> (Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>) {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();

    loop {
        let ret = iter.next();
        match ret {
            None => { break; },
            Some(v) => {
                t0.push(v.0);
                t1.push(v.1);
                t2.push(v.2);
                t3.push(v.3);
            }
        }
    }
    return (t0, t1, t2, t3);
}

pub fn unzip5<T0, T1, T2, T3, T4>(iter: &mut Box<dyn Iterator<Item = (T0,T1,T2,T3,T4)>>) -> (Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>, Vec<T4>) {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();
    let mut t4 = Vec::new();

    loop {
        let ret = iter.next();
        match ret {
            None => { break; },
            Some(v) => {
                t0.push(v.0);
                t1.push(v.1);
                t2.push(v.2);
                t3.push(v.3);
                t4.push(v.4);
            }
        }
    }
    return (t0, t1, t2, t3, t4);
}


#[cfg(test)]
mod tests {
    use crate::itertools::iter::iter_from_vec;

    use super::*;

    #[test]
    fn test1() {
        // let mut data = iter_from_vec(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
        // let ret = unzip(&mut data);
        // assert_eq!(vec!['a', 'b', 'c', 'd'], ret.0);
        // assert_eq!(vec![1, 2, 3, 4], ret.1);


        let mut data = iter_from_vec(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
        let ret = unzip2(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd'], ret.0);
        assert_eq!(vec![1, 2, 3, 4], ret.1);


        let mut data = iter_from_vec(vec![('a', 1, "aa"), ('b', 2, "bb"), ('c', 3, "cc"), ('d', 4, "dd")]);
        let ret = unzip3(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd'], ret.0);
        assert_eq!(vec![1, 2, 3, 4], ret.1);
        assert_eq!(vec!["aa", "bb", "cc", "dd"], ret.2);


        let mut data = iter_from_vec(vec![('a', 1, "aa", 1usize), ('b', 2, "bb", 2usize), ('c', 3, "cc", 3usize), ('d', 4, "dd", 4usize)]);
        let ret = unzip4(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd'], ret.0);
        assert_eq!(vec![1, 2, 3, 4], ret.1);
        assert_eq!(vec!["aa", "bb", "cc", "dd"], ret.2);
        assert_eq!(vec![1usize, 2usize, 3usize, 4usize], ret.3);


        let mut data = iter_from_vec(vec![
            ('a', 1, "aa", 1usize, 1i64), 
            ('b', 2, "bb", 2usize, 2i64), 
            ('c', 3, "cc", 3usize, 3i64), 
            ('d', 4, "dd", 4usize, 4i64),
            ('e', 5, "ee", 5usize, 5i64)
        ]);
        let ret = unzip5(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd', 'e'], ret.0);
        assert_eq!(vec![1, 2, 3, 4, 5], ret.1);
        assert_eq!(vec!["aa", "bb", "cc", "dd", "ee"], ret.2);
        assert_eq!(vec![1usize, 2usize, 3usize, 4usize, 5usize], ret.3);
        assert_eq!(vec![1i64, 2i64, 3i64, 4i64, 5i64], ret.4);
    }
}