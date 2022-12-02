use super::{data, Target};

pub fn run() {
    let result: usize = data()
        .into_iter()
        .map(|(a, b)| {
            (
                match a {
                    'A' => Target::Rock,
                    'B' => Target::Paper,
                    'C' => Target::Scissor,
                    _ => unreachable!(),
                },
                match b {
                    'X' => Target::Rock,
                    'Y' => Target::Paper,
                    'Z' => Target::Scissor,
                    _ => unreachable!(),
                },
            )
        })
        .map(|(a, b)| b.diff(&a))
        .sum();
    println!("{}", result)
}
