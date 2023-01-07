use num_complex::Complex;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;

type Elf = Complex<i32>;
type Elves = HashSet<Complex<i32>>;
type Plans = HashMap<Option<Elf>, usize>;

fn parse_input(content: &str) -> Elves {
    let mut elves = Elves::new();
    content.lines().enumerate().for_each(|(x, row)| {
        row.chars().enumerate().for_each(|(y, ch)| {
            if ch == '#' {
                elves.insert(Complex::new(x as i32, y as i32));
            }
        })
    });
    elves
}

fn plan(elf: Elf, directions: &[Elf], adjacent: &[Elf], elves: &Elves) -> Option<Elf> {
    directions
        .iter()
        .find(|&dir| {
            ![Elf::new(0, 0), Elf::new(0, 1), Elf::new(0, -1)]
                .into_iter()
                .any(|x| elves.contains(&(elf + dir + dir * x)))
                && adjacent.iter().any(|adj| elves.contains(&(elf + adj)))
        })
        .map(|dir| elf + dir)
}

fn simulate(elves: &Elves) {
    let mut directions = [
        Elf::new(-1, 0),
        Elf::new(1, 0),
        Elf::new(0, -1),
        Elf::new(0, 1),
    ];
    let adjacent: [Elf; 8] = [
        Elf::new(-1, 0),
        Elf::new(1, 0),
        Elf::new(0, -1),
        Elf::new(0, 1),
        Elf::new(-1, -1),
        Elf::new(1, -1),
        Elf::new(-1, 1),
        Elf::new(1, 1),
    ];

    let mut all_elves = elves.clone();
    for round in 1..1000000 {
        let mut plans = Plans::new();
        all_elves
            .iter()
            .cloned()
            .map(|elf| plan(elf, &directions, &adjacent, &all_elves))
            .for_each(|planned_pos| {
                if let std::collections::hash_map::Entry::Vacant(e) = plans.entry(planned_pos) {
                    e.insert(1usize);
                } else {
                    *plans.get_mut(&planned_pos).unwrap() += 1usize;
                }
            });
        let mut src = Elves::new();
        let mut dst = Elves::new();
        for elf in all_elves.clone() {
            let elfs_plan = plan(elf, &directions, &adjacent, &all_elves);
            if *plans.get(&elfs_plan).unwrap() == 1usize {
                match elfs_plan {
                    Some(plan) => {
                        src.insert(elf);
                        dst.insert(plan);
                    }
                    None => unreachable!("Something went terribly wrong"),
                }
            }
        }

        if src.is_empty() {
            println!("Part two: {}", round);
            break;
        }

        all_elves = all_elves.clone().sub(&src).union(&dst).cloned().collect();
        directions.rotate_left(1);

        if round == 10 {
            let mut x: Vec<i32> = Vec::with_capacity(all_elves.len());
            let mut y: Vec<i32> = Vec::with_capacity(all_elves.len());
            all_elves.iter().for_each(|elf| {
                x.push(elf.re);
                y.push(elf.im);
            });
            x.sort();
            y.sort();
            let score = (x.last().unwrap() - x.first().unwrap() + 1)
                * (y.last().unwrap() - y.first().unwrap() + 1)
                - all_elves.len() as i32;
            println!("Part one: {}", score)
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let elves = parse_input(&contents);
    simulate(&elves);
}
