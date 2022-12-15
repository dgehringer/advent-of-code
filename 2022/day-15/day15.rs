use regex::Regex;
use std::cmp::max;

type Point = (i64, i64);
type Grid = Vec<(Point, Point)>;

fn parse_input(inp: &str) -> Grid {
    let coord_regex = Regex::new(r"([+-]?\d+)").unwrap();
    inp.lines()
        .map(|l| {
            let matches: Vec<i64> = coord_regex
                .find_iter(l)
                .map(|m| l[m.start()..m.end()].parse::<i64>().unwrap())
                .collect();
            assert_eq!(matches.len(), 4);
            ((matches[0], matches[1]), (matches[2], matches[3]))
        })
        .collect()
}

fn dist(a: &Point, b: &Point) -> i64 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn intervals_for_row(grid: &Grid, target: i64) -> Vec<Point> {
    let mut free_ranges: Vec<Point> = Vec::new();
    for (sensor, nearest_beacon) in grid {
        let nearest_distance = dist(sensor, nearest_beacon);
        let (srow, scol) = *sensor;
        let dcol = (scol - target).abs();
        if dcol <= nearest_distance {
            let drow = nearest_distance - dcol;
            free_ranges.push((srow - drow, srow + drow))
        }
    }
    free_ranges.sort();
    let mut merged_intervals: Vec<Point> = Vec::from_iter(free_ranges.iter().take(1).cloned());
    for (rem_row, rem_col) in free_ranges.iter().skip(1).cloned() {
        let (last_row, last_col) = *merged_intervals.last().unwrap();
        if rem_row > last_col {
            merged_intervals.push((rem_row, rem_col));
        } else {
            *merged_intervals.last_mut().unwrap() = (last_row, max(last_col, rem_col));
        }
    }
    merged_intervals
}

fn part_one(grid: &Grid) {
    let intervals = intervals_for_row(grid, 2000000);
    let (merged_col, merged_row) = intervals.first().unwrap();
    println!("Part 1: {:?}", merged_row - merged_col);
}

fn part_two(grid: &Grid) {
    use rayon::prelude::*;

    let max_row: i64 = 4000000;
    let result: Vec<_> = (0..=max_row)
        .rev()
        .par_bridge()
        .filter_map(|row| {
            let intervals = intervals_for_row(grid, row);
            match intervals.len() {
                0..=1 => None,
                _ => {
                    let (_, col) = *intervals.first().unwrap();
                    Some((col + 1) * max_row + row)
                }
            }
        })
        .collect();
    println!("Part 2: {:?}", result.first().unwrap());
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let grid = parse_input(&contents);
    part_one(&grid);
    part_two(&grid);
}
