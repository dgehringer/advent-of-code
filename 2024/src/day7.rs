mod input;

use crate::input::load_input;
use num_bigint::{BigInt, ToBigInt};
use std::str::FromStr;

type Equation = (BigInt, Vec<BigInt>);

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").take(2).collect();
            (
                BigInt::from_str(parts[0]).unwrap(),
                parts[1]
                    .split_whitespace()
                    .filter_map(|x| BigInt::from_str(x).ok())
                    .collect(),
            )
        })
        .collect()
}

fn solve<T: Eq + PartialOrd>(
    result: &T,
    acc: &T,
    operands: &[T],
    operations: &[Box<dyn Fn(&T, &T) -> T>],
) -> bool {
    if operands.is_empty() {
        return acc == result;
    }
    if acc > result {
        return false;
    };
    operands
        .split_first()
        .map(|(first, rest)| {
            operations.iter().any(|operation| {
                let reduced = operation(acc, first);
                if reduced > *result {return false};
                solve(result, &reduced, rest, operations)
            })
        })
        .unwrap()
}
fn part1(equations: &[Equation]) -> BigInt {
    let zero = 0.to_bigint().unwrap();
    let operations: Vec<Box<dyn Fn(&BigInt, &BigInt) -> BigInt>> = vec![
        Box::new(|a: &BigInt, b: &BigInt| a + b),
        Box::new(|a: &BigInt, b: &BigInt| a * b),
    ];
    equations
        .iter()
        .filter_map(|(result, operands)| {
            if solve(result, &zero, operands, &operations) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

fn part2(equations: &[Equation]) -> BigInt {
    let zero = 0.to_bigint().unwrap();
    let operations: Vec<Box<dyn Fn(&BigInt, &BigInt) -> BigInt>> = vec![
        Box::new(|a: &BigInt, b: &BigInt| a + b),
        Box::new(|a: &BigInt, b: &BigInt| a * b),
        Box::new(|a: &BigInt, b: &BigInt| BigInt::from_str(format!("{}{}", a, b).as_str()).unwrap()),
    ];
    equations
        .iter()
        .filter_map(|(result, operands)| {
            if solve(result, &zero, operands, &operations) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = load_input(2024, 7, None);
    let equations = parse_input(&input);
    println!("Part 1: {:?}", part1(&equations));
    println!("Part 2: {:?}", part2(&equations));
}
