use std::ops::Deref;

use once_cell::sync::Lazy;

pub mod p1;
pub mod p2;

const RAW_DATA: &'static str = include_str!("./data");

pub fn data() -> &'static Vec<Vec<u8>> {
    static DATA: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
        RAW_DATA
            .trim()
            .split("\n")
            .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
            .collect()
    });
    DATA.deref()
}
