use regex::Regex;
use std::collections::HashMap;

type Stacks = HashMap<usize, Vec<char>>;
type Instruction = (usize, usize, usize);

fn parse_input(inp: &str) -> (Stacks, Vec<Instruction>) {
    let mut main_sep = inp.split("\n\n");
    let stack_map: Vec<&str> = main_sep.next().unwrap().lines().collect();
    let regex_stack_ids = Regex::new(r"(\d+)").unwrap();
    let stack_ids: Vec<usize> = regex_stack_ids
        .find_iter(stack_map.last().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();
    let mut stacks: Stacks = HashMap::from_iter(stack_ids.iter().map(|sid| (*sid, Vec::new())));
    let create_regex = Regex::new(r"(\s{3}|\[[A-Z]\])\s?").unwrap();
    for &line in stack_map.iter().take(stack_map.len() - 1) {
        for (stack_id, m) in stack_ids.iter().zip(create_regex.find_iter(line)) {
            if !m.as_str().trim().is_empty() {
                stacks
                    .get_mut(stack_id)
                    .expect("Invalid stack Id")
                    .push(*(&m.as_str()[1..2].chars().next().unwrap()));
            }
        }
    }
    let instructions = main_sep.next().unwrap();
    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|l| {
            let v: Vec<usize> = l
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            (v[0], v[1], v[2])
        })
        .collect();
    (stacks, instructions)
}

fn upper_crates(stacks: &Stacks) -> Vec<char> {
    (1..stacks.keys().max().unwrap() + 1)
        .map(|id| stacks.get(&(id as usize)).unwrap().get(0).unwrap())
        .cloned()
        .collect()
}

fn part_one(stacks: &mut Stacks, instructions: Vec<Instruction>) {
    for (amount, src, dest) in instructions {
        for _ in 0..amount {
            let cr = stacks.get_mut(&src).unwrap().remove(0);
            stacks.get_mut(&dest).unwrap().insert(0, cr);
        }
    }
    println!("Part 1: {:?}", upper_crates(&stacks));
}

fn part_two(stacks: &mut Stacks, instructions: Vec<Instruction>) {
    for (amount, src, dst) in instructions {
        let crates: Vec<char> = (0..amount)
            .map(|i| stacks.get_mut(&src).unwrap().remove(0))
            .collect();
        crates
            .iter()
            .rev()
            .for_each(|&c| stacks.get_mut(&dst).unwrap().insert(0, c))
    }
    println!("Part 2: {:?}", upper_crates(&stacks));
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let (stacks, instructions) = parse_input(&contents);
    part_one(&mut stacks.clone(), instructions.clone());
    part_two(&mut stacks.clone(), instructions);
}
