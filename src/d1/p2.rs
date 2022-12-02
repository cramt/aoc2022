use crate::d1::data;

pub fn run() {
    let mut maxs: Vec<usize> = data()
        .into_iter()
        .map(|x| x.into_iter().sum::<usize>())
        .collect();
    maxs.sort();
    let top_3: usize = maxs.into_iter().rev().take(3).sum();
    println!("{top_3}");
}
