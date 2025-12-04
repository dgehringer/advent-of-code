mod input;

use crate::input::load_input;
use itertools::Itertools;
use std::fs;
use std::ops::Range;

type Instruction = (char, isize);

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .filter_map(|part| {
            let mut split = part.split('-');
            let start = split.next()?.parse::<usize>().ok()?;
            let end = split.next()?.parse::<usize>().ok()?;
            Some((start, end))
        })
        .collect()
}

fn part1(ranges: &[(usize, usize)]) -> usize {
    ranges
        .iter()
        .flat_map(|&(start, end)| {
            (start..=end).filter(|v| {
                if v.ilog10() % 2 == 1 {
                    let v_str = &v.to_string();
                    let (left, right) = v_str.split_at(v_str.len() / 2);
                    left == right
                } else {
                    false
                }
            })
        })
        .sum()
}

fn part2(ranges: &[(usize, usize)]) -> usize {
    ranges
        .iter()
        .flat_map(|&(start, end)| {
            (start..=end).filter(|v| {
                let v_str = v.to_string();
                (1..=v_str.len() / 2).any(|i| {
                    let chunks: Vec<String> = v_str
                        .chars()
                        .chunks(i)
                        .into_iter()
                        .map(|chunk| chunk.collect())
                        .collect();
                    let first = &chunks[0];
                    chunks.iter().all(|c| c == first)
                })
            })
        })
        .sum()
}

fn main() {
    let input = load_input(2025, 2, None);
    let ranges = parse_input(&input);
    println!("Part 1: {:?}", part1(&ranges));
    println!("Part 1: {:?}", part2(&ranges));
}
