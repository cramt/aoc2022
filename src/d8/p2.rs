use std::ops::Range;

use super::data;

fn check_x<T: Iterator<Item = usize>>(
    data: &Vec<Vec<u8>>,
    x_range: T,
    y: usize,
    value: u8,
) -> usize {
    let mut i = 0;
    for x in x_range {
        i += 1;
        if data[x][y] >= value {
            return i;
        }
    }
    i
}

fn check_y<T: Iterator<Item = usize>>(
    data: &Vec<Vec<u8>>,
    x: usize,
    y_range: T,
    value: u8,
) -> usize {
    let mut i = 0;
    for y in y_range {
        i += 1;
        if data[x][y] >= value {
            return i;
        }
    }
    i
}

pub fn run() {
    let data = data();
    let x_max = data.len();
    let y_max = data[0].len();
    let mut vec = Vec::with_capacity(x_max * y_max);
    for x in 0..x_max {
        for y in 0..y_max {
            let value = data[x][y];
            let score = check_x(data, (0..x).rev(), y, value)
                * check_y(data, x, (0..y).rev(), value)
                * check_x(data, (x + 1)..x_max, y, value)
                * check_y(data, x, (y + 1)..y_max, value);
            vec.push(score)
        }
    }
    vec.sort();
    println!("{:?}", vec.last().unwrap());
}
