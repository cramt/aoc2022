use std::collections::HashSet;

use super::data;

pub fn run() {
    let s = data();

    for i in 0..(s.len() - 13) {
        let entry = &s[i..(i + 14)];
        let unqiue_length = entry.chars().collect::<HashSet<_>>().len();
        if unqiue_length == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}
