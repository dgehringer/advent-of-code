use std::fs;

fn parse_input(content: &str) -> Vec<Vec<usize>>{
    content
        .split("\n\n")
        .map(|elf_block|{
            elf_block
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.parse::<usize>().expect("Failed to parse calories"))
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn part_one(calories: &[Vec<usize>]) {
    let max_calories = calories.iter().map(|elf| elf.iter().sum::<usize>()).max().unwrap();
    println!("Part1: {:?}", max_calories);
}

fn part_two(calories: &[Vec<usize>]) {
    let mut calories_per_elf: Vec<usize> = calories.iter().map(|elf| elf.iter().sum::<usize>()).collect();
    calories_per_elf.sort();
    println!("Part2: {:?}", calories_per_elf.iter().rev().take(3).sum::<usize>());
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read puzzle input");
    let calories = parse_input(&contents);
    part_one(&calories);
    part_two(&calories);
}