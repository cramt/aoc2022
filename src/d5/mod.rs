use std::{ops::Deref, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

pub mod p1;
pub mod p2;

const RAW_DATA: &'static str = include_str!("./data");

pub type Stack<T> = Vec<T>;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn stack_data() -> &'static Vec<Stack<char>> {
    static STACK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\s{3}|\[[A-Z]\])").unwrap());
    static STACK_DATA: Lazy<Vec<Stack<char>>> = Lazy::new(|| {
        let raw_data = RAW_DATA.split("\n\n").next().unwrap();
        let matrix = raw_data
            .lines()
            .filter(|x| !x.starts_with(" 1"))
            .map(|x| {
                x.chars()
                    .enumerate()
                    .filter(|(i, _)| *i > 0 && (i - 1) % 4 == 0)
                    .map(|(_, c)| if c == ' ' { None } else { Some(c) })
                    .collect::<Vec<Option<char>>>()
            })
            .collect::<Vec<Vec<Option<char>>>>();
        let matrix = transpose(matrix);
        let matrix = matrix
            .into_iter()
            .map(|x| {
                let mut vec = x.into_iter().flatten().collect::<Vec<_>>();
                vec.reverse();
                vec
            })
            .collect();
        matrix
    });
    STACK_DATA.deref()
}

pub fn operation_data() -> &'static Vec<Operation> {
    static OPERATION_DATA: Lazy<Vec<Operation>> = Lazy::new(|| {
        let mut split = RAW_DATA.trim().split("\n\n");
        split.next().unwrap();
        let raw_data = split.next().unwrap();
        raw_data
            .split("\n")
            .map(|x| x.parse::<Operation>().unwrap())
            .collect()
    });
    OPERATION_DATA.deref()
}

#[derive(Debug)]
pub struct Operation {
    amount: usize,
    from: usize,
    to: usize,
}

impl Operation {
    pub fn perform<T>(&self, mut stacks: Vec<Stack<T>>) -> Vec<Stack<T>> {
        for _ in 0..self.amount {
            let popped = stacks[self.from].pop().unwrap();
            stacks[self.to].push(popped);
        }
        stacks
    }

    pub fn perform_multiple<T>(&self, mut stacks: Vec<Stack<T>>) -> Vec<Stack<T>> {
        let mut to_add = Vec::with_capacity(self.amount);
        for _ in 0..self.amount {
            to_add.push(stacks[self.from].pop().unwrap());
        }
        to_add.reverse();
        for el in to_add {
            stacks[self.to].push(el);
        }
        stacks
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());
        let caps = REGEX.deref().captures(s).ok_or(())?;
        let amount = caps[1].parse().map_err(|_| ())?;
        let from: usize = caps[2].parse().map_err(|_| ())?;
        let from = from - 1;
        let to: usize = caps[3].parse().map_err(|_| ())?;
        let to = to - 1;
        Ok(Self { amount, from, to })
    }
}
