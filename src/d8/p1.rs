use std::ops::Range;

use super::data;

fn check_x(data: &Vec<Vec<u8>>, x_range: Range<usize>, y: usize, value: u8) -> bool {
    for x in x_range {
        if data[x][y] >= value {
            return false;
        }
    }
    true
}

fn check_y(data: &Vec<Vec<u8>>, x: usize, y_range: Range<usize>, value: u8) -> bool {
    for y in y_range {
        if data[x][y] >= value {
            return false;
        }
    }
    true
}

pub fn run() {
    let data = data();
    let x_max = data.len();
    let y_max = data[0].len();
    let mut i = 0usize;
    for x in 0..x_max {
        for y in 0..y_max {
            let value = data[x][y];
            if check_x(data, 0..x, y, value)
                || check_y(data, x, 0..y, value)
                || check_x(data, (x + 1)..x_max, y, value)
                || check_y(data, x, (y + 1)..y_max, value)
            {
                i += 1;
            }
        }
    }
    println!("{i}");
}
