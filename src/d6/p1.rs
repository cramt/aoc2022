use std::collections::HashSet;

use super::data;

pub fn run() {
    let s = data();

    for i in 0..(s.len() - 3) {
        let entry = &s[i..(i + 4)];
        let unqiue_length = entry.chars().collect::<HashSet<_>>().len();
        if unqiue_length == 4 {
            println!("{}", i + 4);
            break;
        }
    }
}
