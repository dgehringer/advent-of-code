mod input;
mod point;

use crate::input::load_input;
use point::Point;
use std::collections::HashSet;

type Field = HashSet<Point>;

fn parse_input(input: &str) -> Field {

        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '@' => Some(Point(x as i32, y as i32)),
                    _ => None,
                })
            })
            .collect()

}

fn accessible(rolls: &Field) -> HashSet<Point> {
    rolls.iter().cloned().filter( |p| {
        (-1..=1).flat_map(move |dx| {
            (-1..=1).filter(move | &dy| {
                if (dx, dy) == (0, 0) {false} else {rolls.contains(&Point(p.0 + dx, p.1 + dy))}
            })
        }).count() < 4
    }).collect::<HashSet<Point>>()
}

fn part1(field: &Field) -> i32 {
    accessible(field).iter().count() as i32
}

fn part2(field: &Field) -> i32 {
    let mut removed = 0;
    let mut rolls = field.clone();
    let mut accessible_points = accessible(&rolls);
    while !accessible_points.is_empty() {
        accessible_points.iter().for_each(|p| {
            rolls.remove(p);
            removed += 1;
        });
        accessible_points = accessible(&rolls);
    }
    removed
}

fn main() {
    let input = load_input(2025, 4, None);
    //let input = fs::read_to_string("test").unwrap();
    let field = parse_input(&input);
    println!("Part 1: {:?}", part1(&field));
    println!("Part 2: {:?}", part2(&field));
}
