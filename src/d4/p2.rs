use super::{data, RangeExt};

pub fn run() {
    let data = data();
    let result = data
        .into_iter()
        .filter(|(a, b)| a.is_overlap(b))
        .collect::<Vec<_>>()
        .len();
    println!("{result}");
}
