use super::{contains_both, grouped_data, numeric};

pub fn run() {
    let result = grouped_data()
        .into_iter()
        .map(|x| {
            numeric(
                contains_both(
                    contains_both(x[0].chars().collect(), x[1].chars().collect())
                        .into_iter()
                        .collect(),
                    x[2].chars().collect(),
                )
                .into_iter()
                .next()
                .unwrap(),
            )
            .unwrap()
        })
        .sum::<usize>();
    println!("{result:?}");
}
