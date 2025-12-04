mod input;

use crate::input::load_input;
use itertools::Itertools;
use std::fs;
use std::ops::Range;

type Banks = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Banks {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn joltage(banks: &Banks, batteries: usize) -> impl Iterator<Item = usize> {
    banks.iter().map(move |bank|{
        let mut jolt: String = String::with_capacity(batteries);
        let mut max_index = 0usize;
        for i in (0..batteries).rev() {
            let max_battery = bank[max_index..(bank.len() - i)].iter().max().unwrap();
            max_index += bank[max_index..(bank.len() - i)].iter().position(|&b| b == *max_battery).unwrap() + 1;
            jolt.push((*max_battery).to_string().chars().next().unwrap());
        }
        println!("{}", jolt);
        jolt.parse::<usize>().unwrap()
    })
}


fn main() {
    let input = load_input(2025, 3, None);
    let pack = parse_input(&input);

    println!("Part 1: {:?}", joltage(&pack, 2).sum::<usize>());
    println!("Part 2: {:?}", joltage(&pack, 12).sum::<usize>());
}
