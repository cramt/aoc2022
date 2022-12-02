use std::ops::Deref;

use once_cell::sync::Lazy;

pub mod p1;
pub mod p2;

const RAW_DATA: &'static str = include_str!("./data");

static DATA: Lazy<Vec<Vec<usize>>> = Lazy::new(|| {
    RAW_DATA
        .trim()
        .split("\r\n\r\n")
        .map(|x| x.split("\r\n").map(|x| x.trim().parse().unwrap()).collect())
        .collect()
});

pub fn data() -> &'static Vec<Vec<usize>> {
    DATA.deref()
}
