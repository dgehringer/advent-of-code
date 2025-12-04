
mod input;

use std::fs;
use std::ops::Range;
use crate::input::load_input;

type Instruction = (char, isize);

fn parse_input(input: &str) -> Vec<Range<usize>> {
    input.lines().next().unwrap().split(", ").filter_map(|part| {
        let mut split = part.split('-');
        let start = split.next()?.parse::<usize>().ok()?;
        let end = split.next()?.parse::<usize>().ok()?;
        Some(start..end)
    }).collect()
}

fn part1(instructions: &[Instruction]) -> isize {
    let mut clicks = 0isize;
    let mut dial = 50isize;
    for (char, amount) in instructions {
        dial = match char {
            'L' => (dial - amount),
            'R' => (dial + amount),
            _ => unimplemented!(),
        } % 100;
        if dial < 0 {
            dial += 100;
        }
        if dial == 0 {
            clicks += 1;
        }
    }
    clicks
}

fn part2(instructions: &[Instruction]) -> isize {
    let mut clicks = 0isize;
    let dial = 50isize;
    for (char, amount) in instructions {
        clicks += match char {
            'L' => dial+1..=dial+amount,
            'R' => dial-1..=dial-amount,
            _ => unimplemented!(),
        }.filter(|d| d % 100 == 0).count() as isize
    }
    clicks * 2 - 1
}

fn main() {
    let input = load_input(2025, 2, None);
    let input = fs::read_to_string("test").unwrap();
    let ranges = parse_input(&input);
    println!("Part 1: {:?}", ranges);


}