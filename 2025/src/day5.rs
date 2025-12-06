mod input;

use crate::input::load_input;
use itertools::Itertools;

type Database = (Vec<(usize, usize)>, Vec<usize>);

fn parse_input(input: &str) -> Database {
    let (ranges, ids) = input.split("\n\n").collect_tuple().unwrap();
    (
        ranges
            .lines()
            .map(|l| {
                let mut split = l.split('-');
                let start = split.next().unwrap().parse::<usize>().unwrap();
                let end = split.next().unwrap().parse::<usize>().unwrap();
                (start, end)
            })
            .collect(),
        ids.lines().map(|l| l.parse::<usize>().unwrap()).collect(),
    )
}

fn part1(db: &Database) -> usize {
    let (ranges, ids) = db;
    ids.iter()
        .filter(|&id| {
            ranges
                .iter()
                .any(|(start, end)| *id >= *start && *id <= *end)
        })
        .count()
}

fn part2(db: &Database) -> usize {
    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut remaining = db.0.clone();
    while let Some((start, end)) = remaining.pop() {
        if let Some((pos, &overlap)) = remaining
            .iter()
            .find_position(|&(ostart, oend)| *ostart <= end && start <= *oend)
        {
            let (ostart, oend) = overlap;
            remaining.remove(pos);
            remaining.push((start.min(ostart), end.max(oend)));
        } else {
            ranges.push((start, end));
        }
    }
    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    let input = load_input(2025, 5, None);
    let db = parse_input(&input);
    println!("Part 1: {:?}", part1(&db));
    println!("Part 2: {:?}", part2(&db));
}
