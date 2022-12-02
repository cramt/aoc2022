use crate::d1::data;

pub fn run() {
    let max = data()
        .into_iter()
        .map(|x| x.into_iter().sum::<usize>())
        .max()
        .unwrap();
    println!("{max}");
}
