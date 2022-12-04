pub mod p1;
pub mod p2;

use std::ops::{Deref, RangeInclusive};

use once_cell::sync::Lazy;

use regex::Regex;

const RAW_DATA: &'static str = include_str!("./data");

trait RangeExt {
    fn is_subset(&self, rhs: &Self) -> bool;

    fn is_superset(&self, rhs: &Self) -> bool {
        rhs.is_subset(self)
    }

    fn is_overlap(&self, rhs: &Self) -> bool;
}

impl<T: Ord> RangeExt for RangeInclusive<T> {
    fn is_subset(&self, rhs: &Self) -> bool {
        self.start() <= rhs.start() && self.end() >= rhs.end()
    }

    fn is_overlap(&self, rhs: &Self) -> bool {
        let (a, b) = if rhs.start() > self.start() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        b.end() >= a.start()
    }
}

fn parse_range(s: &str) -> RangeInclusive<usize> {
    static INCLUSIVE_RANGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\-(\d+)").unwrap());
    let captures = INCLUSIVE_RANGE_REGEX
        .deref()
        .captures_iter(s)
        .next()
        .unwrap();
    let first: usize = captures[1].parse().unwrap();
    let second: usize = captures[2].parse().unwrap();
    first..=second
}

fn data() -> &'static Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    static DATA: Lazy<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>> = Lazy::new(|| {
        RAW_DATA
            .trim()
            .split("\n")
            .map(|x| {
                let mut split = x.split(",");
                let first = split.next().unwrap();
                let second = split.next().unwrap();
                (parse_range(first), parse_range(second))
            })
            .collect()
    });
    DATA.deref()
}
