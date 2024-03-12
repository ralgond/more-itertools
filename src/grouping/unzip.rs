use crate::error::Error;



pub fn unzip2<T0, T1>(iter: &mut Box<dyn Iterator<Item = Result<(T0,T1),Error>>>) -> Result<(Vec<T0>, Vec<T1>),Error> 
where T0: 'static, T1: 'static

{
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();

    loop {
        let ret = iter.next();
        if let Some(v) = ret {
            match v {
                Ok(ok_v) => {
                    t0.push(ok_v.0);
                    t1.push(ok_v.1);
                },
                Err(err_v) => { // upstream error
                    return Err(err_v);
                }
            }
        } else {
            break;
        }
    }
    return Ok((t0, t1));
}

pub fn unzip3<T0, T1, T2>(iter: &mut Box<dyn Iterator<Item = Result<(T0,T1,T2),Error>>>) -> Result<(Vec<T0>, Vec<T1>, Vec<T2>),Error> {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();

    loop {
        let ret = iter.next();
        if let Some(v) = ret {
            match v {
                Ok(ok_v) => {
                    t0.push(ok_v.0);
                    t1.push(ok_v.1);
                    t2.push(ok_v.2);
                },
                Err(err_v) => { // upstream error
                    return Err(err_v);
                }
            }
        } else {
            break;
        }
    }
    return Ok((t0, t1, t2));
}

pub fn unzip4<T0, T1, T2, T3>(iter: &mut Box<dyn Iterator<Item = Result<(T0,T1,T2,T3),Error>>>) -> Result<(Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>),Error> {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();

    loop {
        let ret = iter.next();
        if let Some(v) = ret {
            match v {
                Ok(ok_v) => {
                    t0.push(ok_v.0);
                    t1.push(ok_v.1);
                    t2.push(ok_v.2);
                    t3.push(ok_v.3);
                },
                Err(err_v) => { // upstream error
                    return Err(err_v);
                }
            }
        } else {
            break;
        }
    }
    return Ok((t0, t1, t2, t3));
}

pub fn unzip5<T0, T1, T2, T3, T4>(iter: &mut Box<dyn Iterator<Item = Result<(T0,T1,T2,T3,T4),Error>>>) -> Result<(Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>, Vec<T4>),Error> {
    let mut t0 = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();
    let mut t4 = Vec::new();

    loop {
        let ret = iter.next();
        if let Some(v) = ret {
            match v {
                Ok(ok_v) => {
                    t0.push(ok_v.0);
                    t1.push(ok_v.1);
                    t2.push(ok_v.2);
                    t3.push(ok_v.3);
                    t4.push(ok_v.4);
                },
                Err(err_v) => { // upstream error
                    return Err(err_v);
                }
            }
        } else {
            break;
        }
    }
    return Ok((t0, t1, t2, t3, t4));
}

#[cfg(test)]
mod tests {
    use crate::{error, itertools::iter::iter_from_vec, utils::{generate_okok_iterator, generate_okokerr_iterator}};

    use super::*;

    #[test]
    fn test1() {
        // let mut data = iter_from_vec(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
        // let ret = unzip(&mut data);
        // assert_eq!(vec!['a', 'b', 'c', 'd'], ret.0);
        // assert_eq!(vec![1, 2, 3, 4], ret.1);


        let mut data = generate_okok_iterator(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
        let ret = unzip2(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd'], ret.as_ref().ok().unwrap().0);
        assert_eq!(vec![1, 2, 3, 4], ret.as_ref().ok().unwrap().1);


        let mut data = generate_okok_iterator(vec![('a', 1, "aa"), ('b', 2, "bb"), ('c', 3, "cc"), ('d', 4, "dd")]);
        let ret = unzip3(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd'], ret.as_ref().ok().unwrap().0);
        assert_eq!(vec![1, 2, 3, 4], ret.as_ref().ok().unwrap().1);
        assert_eq!(vec!["aa", "bb", "cc", "dd"], ret.as_ref().ok().unwrap().2);


        let mut data = generate_okok_iterator(vec![('a', 1, "aa", 1usize), ('b', 2, "bb", 2usize), ('c', 3, "cc", 3usize), ('d', 4, "dd", 4usize)]);
        let ret = unzip4(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd'], ret.as_ref().ok().unwrap().0);
        assert_eq!(vec![1, 2, 3, 4], ret.as_ref().ok().unwrap().1);
        assert_eq!(vec!["aa", "bb", "cc", "dd"], ret.as_ref().ok().unwrap().2);
        assert_eq!(vec![1usize, 2usize, 3usize, 4usize], ret.as_ref().ok().unwrap().3);


        let mut data = generate_okok_iterator(vec![
            ('a', 1, "aa", 1usize, 1i64), 
            ('b', 2, "bb", 2usize, 2i64), 
            ('c', 3, "cc", 3usize, 3i64), 
            ('d', 4, "dd", 4usize, 4i64),
            ('e', 5, "ee", 5usize, 5i64)
        ]);
        let ret = unzip5(&mut data);
        assert_eq!(vec!['a', 'b', 'c', 'd', 'e'], ret.as_ref().ok().unwrap().0);
        assert_eq!(vec![1, 2, 3, 4, 5], ret.as_ref().ok().unwrap().1);
        assert_eq!(vec!["aa", "bb", "cc", "dd", "ee"], ret.as_ref().ok().unwrap().2);
        assert_eq!(vec![1usize, 2usize, 3usize, 4usize, 5usize], ret.as_ref().ok().unwrap().3);
        assert_eq!(vec![1i64, 2i64, 3i64, 4i64, 5i64], ret.as_ref().ok().unwrap().4);
    }

    #[test]
    fn test2() {
        let mut data = generate_okokerr_iterator(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)], error::overflow_error("[test]".to_string()));
        let ret = unzip2(&mut data);
        assert_eq!(error::Kind::OverflowError, ret.as_ref().err().unwrap().kind());


        let mut data = generate_okokerr_iterator(vec![('a', 1, "aa"), ('b', 2, "bb"), ('c', 3, "cc"), ('d', 4, "dd")], error::overflow_error("[test]".to_string()));
        let ret = unzip3(&mut data);
        assert_eq!(error::Kind::OverflowError, ret.as_ref().err().unwrap().kind());


        let mut data = generate_okokerr_iterator(vec![('a', 1, "aa", 1usize), ('b', 2, "bb", 2usize), ('c', 3, "cc", 3usize), ('d', 4, "dd", 4usize)], error::overflow_error("[test]".to_string()));
        let ret = unzip4(&mut data);
        assert_eq!(error::Kind::OverflowError, ret.as_ref().err().unwrap().kind());


        let mut data = generate_okokerr_iterator(vec![
            ('a', 1, "aa", 1usize, 1i64), 
            ('b', 2, "bb", 2usize, 2i64), 
            ('c', 3, "cc", 3usize, 3i64), 
            ('d', 4, "dd", 4usize, 4i64),
            ('e', 5, "ee", 5usize, 5i64),
        ], error::overflow_error("[test]".to_string()));
        let ret = unzip5(&mut data);
        assert_eq!(error::Kind::OverflowError, ret.as_ref().err().unwrap().kind());
    }
}