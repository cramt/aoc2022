use std::hash::Hash;
use std::{collections::HashSet, ops::Deref};

use once_cell::sync::Lazy;

pub mod p1;
pub mod p2;

const RAW_DATA: &'static str = include_str!("./data");

pub fn data() -> &'static Vec<(&'static str, &'static str)> {
    static DATA: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
        RAW_DATA
            .trim()
            .split("\n")
            .map(|x| {
                let length = x.len();
                if length % 2 != 0 {
                    panic!("not even size")
                }
                let half_length = length / 2;
                (&x[0..half_length], &x[half_length..length])
            })
            .collect()
    });
    DATA.deref()
}

pub fn grouped_data() -> &'static Vec<[&'static str; 3]> {
    static DATA: Lazy<Vec<[&'static str; 3]>> = Lazy::new(|| {
        let data = RAW_DATA.trim().split("\n").collect::<Vec<&str>>();
        let length = data.len() / 3;
        let mut vec = Vec::with_capacity(length);
        let mut iter = data.into_iter();
        for _ in 0..(length) {
            let mut slice = [""; 3];
            slice[0] = iter.next().unwrap();
            slice[1] = iter.next().unwrap();
            slice[2] = iter.next().unwrap();
            vec.push(slice);
        }
        vec
    });
    DATA.deref()
}

pub fn numeric(c: char) -> Option<usize> {
    if ('a'..='z').contains(&c) {
        return Some((c as usize) - 96);
    }
    if ('A'..='Z').contains(&c) {
        return Some((c as usize) - 38);
    }
    return None;
}

pub fn contains_both<T>(a: Vec<T>, b: Vec<T>) -> HashSet<T>
where
    T: Eq,
    T: Hash,
{
    a.into_iter().filter(|x| b.contains(x)).collect()
}

#[cfg(test)]
mod test {
    mod numeric {
        use crate::d3::numeric;

        #[test]
        fn lowercase() {
            assert_eq!(1, numeric('a').unwrap());
            assert_eq!(26, numeric('z').unwrap());
        }

        #[test]
        fn uppercase() {
            assert_eq!(27, numeric('A').unwrap());
            assert_eq!(52, numeric('Z').unwrap());
        }
    }
}
