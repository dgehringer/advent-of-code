use std::fs;

fn parse_input(content: &str) -> Vec<usize>{
    content
        .split("\n\n")
        .map(|elf_block|{
            elf_block
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.parse::<usize>().expect("Failed to parse calories"))
                .sum::<usize>()
        })
        .collect()
}

fn part_one(calories_per_elf: &[usize]) {
    println!("Part1: {:?}", calories_per_elf.iter().max().unwrap());
}

fn part_two(calories_per_elf: &[usize]) {
    println!("Part2: {:?}", calories_per_elf.iter().rev().take(3).sum::<usize>());
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read puzzle input");
    let mut calories_per_elf = parse_input(&contents);
    calories_per_elf.sort();
    part_one(&calories_per_elf);
    part_two(&calories_per_elf);
}