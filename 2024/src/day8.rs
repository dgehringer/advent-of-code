mod input;

use crate::input::load_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, Eq, PartialOrd, Hash, PartialEq, Copy, Clone)]
struct Point(i32, i32);
type Antennas = HashMap<char, Vec<Point>>;

#[derive(Debug)]
struct Field {
    antennas: Antennas,
    size: Point,
}

impl FromStr for Field {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antennas: Antennas = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c != '.' {
                        Some((c, Point(x as i32, y as i32)))
                    } else {
                        None
                    }
                })
            })
            .into_group_map();
        let h = s.lines().count() as i32;
        let first_line = s.lines().next().ok_or(String::from("Empty input"))?;
        let w = first_line.chars().count() as i32;
        Ok(Field {
            antennas,
            size: Point(w, h),
        })
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Field {
    fn contains(&self, point: &Point) -> bool {
        point.0 >= 0 && point.0 < self.size.0 && point.1 >= 0 && point.1 < self.size.1
    }
}

fn generate_antinodes(field: &Field, antennas: &Vec<Point>, part_one: bool) -> Vec<Point> {
    let mut points = Vec::with_capacity(2);

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let p = antennas[j] - antennas[i];
            let mut forward = antennas[j] + p;
            if  !part_one {
                points.push(antennas[i]);
                points.push(antennas[j]);
            }
            while field.contains(&forward) {
                points.push(forward);
                forward = forward + p;
                if part_one {break};
            }
            let mut backward = antennas[i] - p;
            while field.contains(&backward) {
                points.push(backward);
                backward = backward - p;
                if part_one {break};
            }
        }
    }
    points
}

fn solve(field: &Field, part_one: bool) -> i32 {
    field
        .antennas
        .values()
        .flat_map(|ant| generate_antinodes(field, ant, part_one))
        .collect::<HashSet<Point>>()
        .len() as i32
}

fn main() {
    let input = load_input(2024, 8, None);
    let field = Field::from_str(&input).unwrap();
    println!("Part 1: {:?}", solve(&field, true));
    println!("Part 2: {:?}", solve(&field, false));
}
