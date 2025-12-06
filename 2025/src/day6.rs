mod input;

use crate::input::load_input;
use itertools::Itertools;
use regex::Regex;

type Problems = (Vec<Vec<usize>>, Vec<char>);

fn parse_input(input: &str) -> Problems {
    let op_regex = Regex::new(r"[+*]").unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();
    (
        input
            .lines()
            .take(input.lines().count() - 1)
            .map(|l| {
                num_regex
                    .find_iter(l)
                    .map(|m| m.as_str().parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
        op_regex
            .find_iter(input.lines().last().unwrap())
            .map(|m| m.as_str().chars().next().unwrap())
            .collect_vec(),
    )
}

fn part1(problems: Problems) -> usize {
    let (numbers, ops) = problems;
    numbers
        .iter()
        .skip(1)
        .fold(numbers.first().unwrap().clone(), |acc, row| {
            acc.iter()
                .zip(row.iter().zip(ops.iter()))
                .map(|(a, (b, op))| match op {
                    '+' => a + b,
                    '*' => a * b,
                    _ => panic!("Unknown operator"),
                })
                .collect_vec()
        }).iter().sum()

}

fn part2(input: &str) -> usize {
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();
    let mut transposed = vec![String::with_capacity(h); w];
    input.lines().enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, c)| {
            transposed[j].push(c);
        });
    });
    let blocks = transposed.split(|s| s.chars().all(|c| c.is_whitespace())).collect_vec();
    let mut total = 0usize;
    for block in blocks {
        let (num, op) = block[0].split_at(block[0].len() - 1);
        let mut acc = num.trim().parse::<usize>().unwrap();
        for line in block.iter().skip(1) {
            let number = line.trim().parse::<usize>().unwrap();
            match op {
                "+" => acc += number,
                "*" => acc *= number,
                _ => panic!("Unknown operator"),
            }
        }
        total += acc;
    }
    total
}

fn main() {
    let input = load_input(2025, 6, None);
    println!("Part 1: {:?}", part1(parse_input(&input)));
    println!("Part 2: {:?}", part2(&input));
}
