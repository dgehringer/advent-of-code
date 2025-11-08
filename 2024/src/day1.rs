mod input;

use crate::input::load_input;
use std::collections::HashMap;

type ParsedInput = (Vec<i32>, Vec<i32>);

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .filter_map(|line| {
            let line = line
                .split_whitespace()
                .take(2)
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            if line.len() == 2 {
                Some((line[0], line[1]))
            } else {
                None
            }
        })
        .fold((Vec::new(), Vec::new()), |(mut a, mut b), (x, y)| {
            a.push(x);
            b.push(y);
            (a, b)
        })
}

fn part1(input: &mut ParsedInput) -> i32 {
    let (a, b) = input;
    a.sort();
    b.sort();
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum()
}

fn part2(input:  &ParsedInput) -> i32 {
    let (left, right) = input;
    let right_hist = right.iter().fold(HashMap::new(), |mut m, &x| {
        *m.entry(x).or_insert(0) += 1;
        m
    });
    left.iter().map(|x| x * right_hist.get(x).unwrap_or(&0)).sum()
}

fn main() {
    let input = load_input(2024, 1, None);
    //let input = fs::read_to_string("test").expect("problem reading input");
    let mut parsed = parse_input(&input);
    println!("Part 1: {:?}", part1(&mut parsed));
    println!("Part 2: {:?}", part2(&parsed));
}
