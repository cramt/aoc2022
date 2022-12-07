use crate::d7::{SIZE_NEEDED, TOTAL_SIZE};

use super::{calc_sum, data};

pub fn run() {
    let data = data();
    let mut sizes = Vec::new();
    let root = data.borrow();
    let (cur_used, sizes) = calc_sum(&root, &mut sizes);
    let needed = SIZE_NEEDED - (TOTAL_SIZE - cur_used);
    println!("{}", *sizes.iter().filter(|x| **x > needed).min().unwrap());
}
