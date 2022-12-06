use std::ops::Deref;

use once_cell::sync::Lazy;

pub mod p1;
pub mod p2;

const RAW_DATA: &'static str = include_str!("./data");

pub fn data() -> &'static str {
    static DATA: Lazy<&str> = Lazy::new(|| RAW_DATA.trim());

    DATA.deref()
}
