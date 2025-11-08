mod input;

use crate::input::load_input;
use num_bigint::{BigInt, ToBigInt};
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

type Instructions = Vec<Instruction>;

fn parse_input(input: &str) -> Instructions {
    let mul_regex = Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap();
    mul_regex
        .captures_iter(input)
        .map(|x| {
            if x[0].starts_with("mul") {
                Instruction::Mul(x[1].parse().unwrap(), x[2].parse().unwrap())
            } else {
                match &x[0] {
                    "do()" => Instruction::Do,
                    "don't()" => Instruction::Dont,
                    _ => panic!("Unexpected match"),
                }
            }
        })
        .collect()
}

fn part1(instructions: &Instructions) -> BigInt {
    instructions
        .iter()
        .fold(0.to_bigint().unwrap(), |acc, instr| match instr {
            Instruction::Mul(a, b) => acc + (a * b),
            _ => acc,
        })
}

fn part2(instructions: &Instructions) -> BigInt {
    instructions
        .iter()
        .fold((0.to_bigint().unwrap(), true), |(acc, execute), instr| {
            match instr {
                Instruction::Do => (acc, true),
                Instruction::Dont => (acc, false),
                Instruction::Mul(a, b) => {
                    if execute {
                        (acc + (a * b), execute)
                    } else {
                        (acc, execute)
                    }
                }
            }
        })
        .0
}

fn main() {
    let input = load_input(2024, 3, None);
    let instructions = parse_input(&input);
    println!("Part 1: {:?}", part1(&instructions));
    println!("Part 2: {:?}", part2(&instructions));
}
