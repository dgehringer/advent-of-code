extern crate core;

use std::collections::HashSet;

fn find_message_start(message: &str, length: usize) -> Option<usize> {
    for i in 0usize..(message.len() - length) {
        let subset = &message[i..i + length];
        if subset.chars().collect::<HashSet<char>>().len() == length {
            return Some(i + length);
        }
    }
    None
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    println!("Part 1: {}", find_message_start(&contents, 4).unwrap());
    println!("Part 2: {}", find_message_start(&contents, 14).unwrap());
}
