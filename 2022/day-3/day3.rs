use itertools::Itertools;
use std::collections::HashSet;

fn score(c: char) -> usize {
    if c.is_lowercase() {
        1usize + (c as usize) - ('a' as usize)
    } else {
        27usize + (c as usize) - ('A' as usize)
    }
}

fn slice_to_hashset(sl: &str) -> HashSet<char> {
    HashSet::from_iter(sl.chars())
}

fn part_one(backpacks: &[&str]) {
    let splitted_backpacks: Vec<(&str, &str)> = backpacks
        .iter()
        .map(|&bp| bp.split_at(bp.len() / 2))
        .collect();

    let final_score: usize = splitted_backpacks
        .iter()
        .map(|&(c1, c2)| {
            let s2 = slice_to_hashset(c2);
            score(*slice_to_hashset(c1).intersection(&s2).next().unwrap())
        })
        .sum();
    println!("Part1 {:?}", final_score);
}

fn part_two(backpacks: &[&str]) {
    let final_score: usize = backpacks
        .iter()
        .tuple_windows::<(_, _, _)>()
        .step_by(3)
        .map(|(a, b, c)| {
            let common: char = *slice_to_hashset(a)
                .intersection(&slice_to_hashset(b))
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&slice_to_hashset(c))
                .next()
                .unwrap();
            score(common)
        })
        .sum();

    println!("Part2 {:?}", final_score);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let backpacks: Vec<&str> = contents.lines().collect();
    part_one(&backpacks);
    part_two(&backpacks);
}
