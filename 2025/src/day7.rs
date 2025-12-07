mod input;
mod point;

use crate::input::load_input;
use itertools::Itertools;
use point::Point;
use std::collections::{HashMap, HashSet};

type Splitter = Point;
type Beam = Point;

type Field = (HashSet<Splitter>, Point);

fn parse_input(input: &str) -> (Beam, Field) {
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();
    let beam = Point(
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap() as i32,
        0,
    );
    (
        beam,
        (
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '^' {
                            Some(Point(x as i32, y as i32))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
            Point(w as i32, h as i32),
        ),
    )
}

fn part1(start: &Beam, field: &Field) -> usize {
    let mut splits = 0;
    let (splitters, size) = field;
    let mut beams = Vec::from([start.clone()]);

    for i in 0..size.1 {
        // propatage beams
        beams = beams
            .iter()
            .flat_map(|beam| {
                let next = beam.clone() + Point(0, 1);
                if splitters.contains(&next) {
                    splits += 1;
                    vec![next.clone() + Point(-1, 0), next + Point(1, 0)]
                } else {
                    vec![next]
                }
            })
            .collect::<HashSet<Point>>()
            .into_iter()
            .collect();
    }
    splits
}

fn part2(start: &Beam, field: &Field) -> usize {
    let mut beams = HashMap::from([(start.clone().0, 1usize)]);
    // beams is a dictionary of x, coordiante and number of beams at that coordinate
    let (splitters, size) = field;
    for y in 0..size.1 {
        let mut next_beams: HashMap<i32, usize> = HashMap::new();
        beams
            .iter()
            .flat_map(|(&x, &nbeams)| {
                if splitters.contains(&Point(x, y)) {
                    vec![(x - 1, nbeams), (x + 1, nbeams)]
                } else {
                    vec![(x, nbeams)]
                }
            })
            .for_each(|(x, nbeams)| {
                *next_beams.entry(x).or_insert(0) += nbeams;
            });

        beams = next_beams;
    }
    beams.values().sum()
}

fn main() {
    let input = load_input(2025, 7, None);
    let (beam, field) = parse_input(&input);
    println!("Part 1: {:?}", part1(&beam, &field));
    println!("Part 2: {:?}", part2(&beam, &field));
}
