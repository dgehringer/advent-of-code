mod input;

use crate::input::load_input;
use itertools::Itertools;

type Report = Vec<i32>;
type Reports = Vec<Report>;

fn parse_input(input: &str) -> Reports {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn classify(report: &Report) -> bool {
    let diffs = report.iter().tuple_windows().map(|(a, b)| b - a);
    let sign = diffs.clone().take(1).next().unwrap().signum();
    !diffs
        .clone()
        .any(|diff| !(diff.abs() > 0 && diff.abs() <= 3))
        && diffs.clone().fold(true, |acc, x| acc && x.signum() == sign)
}
fn part1(reports: &Reports) -> i32 {
    reports.iter().map(|report| if classify(report) { 1 } else { 0 }).sum()
}

fn part2(reports: &Reports) -> i32 {
    reports
        .iter()
        .map(|report| {
            if classify(report) {
                1
            } else {
                if (0..report.len()).any(|index| {
                    let mut modified = report.clone();
                    modified.remove(index);
                    classify(&modified)
                }) {
                    1
                } else {
                    0
                }
            }
        })
        .sum()
}

fn main() {
    let input = load_input(2024, 2, None);
    let parsed = parse_input(&input);
    print!("Part 1: {:?}\n", part1(&parsed));
    print!("Part 1: {:?}\n", part2(&parsed));
}
