use super::{operation_data, stack_data};

pub fn run() {
    let stacks = stack_data().clone();
    let operations = operation_data();
    let stacks = operations
        .iter()
        .fold(stacks, |acc, operation| operation.perform(acc));
    println!(
        "{}",
        stacks
            .into_iter()
            .map(|mut x| x.pop().unwrap().to_string())
            .collect::<String>()
    );
}
