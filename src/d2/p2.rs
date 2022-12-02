use super::{data, Target, VsOutcome};

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
                    'X' => VsOutcome::Loss,
                    'Y' => VsOutcome::Tie,
                    'Z' => VsOutcome::Win,
                    _ => unreachable!(),
                },
            )
        })
        .map(|(opponent, outcome)| opponent.reverse_vs(&outcome).diff(&opponent))
        .sum();
    println!("{}", result)
}
