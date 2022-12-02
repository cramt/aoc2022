use std::ops::Deref;

use once_cell::sync::Lazy;

pub mod p1;
pub mod p2;

const RAW_DATA: &'static str = include_str!("./data");

static DATA: Lazy<Vec<(char, char)>> = Lazy::new(|| {
    RAW_DATA
        .trim()
        .split("\n")
        .map(|x| {
            let mut chars = x.chars();
            let first = chars.next().unwrap();
            chars.next().unwrap();
            let second = chars.next().unwrap();
            (first, second)
        })
        .collect()
});

pub fn data() -> &'static Vec<(char, char)> {
    DATA.deref()
}

#[derive(Clone)]
enum Target {
    Rock,
    Paper,
    Scissor,
}

enum VsOutcome {
    Win,
    Loss,
    Tie,
}

impl Target {
    pub fn vs(&self, rhs: &Self) -> VsOutcome {
        match (self, rhs) {
            (Self::Rock, Self::Scissor) => VsOutcome::Win,
            (Self::Paper, Self::Rock) => VsOutcome::Win,
            (Self::Scissor, Self::Paper) => VsOutcome::Win,

            (Self::Scissor, Self::Rock) => VsOutcome::Loss,
            (Self::Rock, Self::Paper) => VsOutcome::Loss,
            (Self::Paper, Self::Scissor) => VsOutcome::Loss,

            _ => VsOutcome::Tie,
        }
    }

    pub fn reverse_vs(&self, outcome: &VsOutcome) -> Self {
        match (self, outcome) {
            (Self::Rock, VsOutcome::Win) => Self::Paper,
            (Self::Rock, VsOutcome::Loss) => Self::Scissor,
            (Self::Paper, VsOutcome::Win) => Self::Scissor,
            (Self::Paper, VsOutcome::Loss) => Self::Rock,
            (Self::Scissor, VsOutcome::Win) => Self::Rock,
            (Self::Scissor, VsOutcome::Loss) => Self::Paper,
            (x, VsOutcome::Tie) => x.clone(),
        }
    }

    pub fn numeric(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    pub fn diff(&self, rhs: &Self) -> usize {
        self.outcome_diff(&self.vs(rhs))
    }

    pub fn outcome_diff(&self, outcome: &VsOutcome) -> usize {
        match outcome {
            VsOutcome::Win => 6 + self.numeric(),
            VsOutcome::Loss => self.numeric(),
            VsOutcome::Tie => 3 + self.numeric(),
        }
    }
}
