use crate::d7::SMALL_FILE_LIMIT;

use super::{calc_sum, data};

pub fn run() {
    let data = data();
    let mut sizes = Vec::new();
    let root = data.borrow();
    let (_, sizes) = calc_sum(&root, &mut sizes);
    println!(
        "{}",
        sizes
            .iter()
            .filter(|x| **x < SMALL_FILE_LIMIT)
            .sum::<usize>()
    );
}

