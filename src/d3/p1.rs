use std::collections::HashSet;

use super::{contains_both, data, numeric};

pub fn run() {
    let shared = data()
        .into_iter()
        .map(|x| contains_both(x.0.chars().collect(), x.1.chars().collect()))
        .collect::<Vec<HashSet<char>>>();
    let result = shared
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(numeric)
                .map(|x| x.unwrap() as usize)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{result}")
}
