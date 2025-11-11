mod input;

use crate::input::load_input;
use num_complex::Complex;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

type Point = Complex<i32>;

#[derive(Debug, Clone)]
struct Field {
    height: Range<i32>,
    width: Range<i32>,
    start: Point,
    obstacles: HashSet<Point>,
    space: HashSet<Point>,
}

#[derive(Debug, Clone, Copy)]
pub struct Guard {
    pos: Point,
    dir: Point,
}
fn parse_input(input: &str) -> Field {
    let mut map: HashMap<char, HashSet<Point>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map.entry(c)
                .or_default()
                .insert(Point::new(x as i32, y as i32));
        })
    });

    Field {
        start: map[&'^'].iter().next().cloned().unwrap(),
        space: map[&'.'].clone(),
        obstacles: map[&'#'].clone(),
        width: 0..input.lines().count() as i32,
        height: 0..input.lines().last().unwrap().chars().count() as i32,
    }
}

impl Field {
    fn with_obstacle(&self, obstacle: Point) -> Field {
        let mut new_field = self.clone();
        new_field.obstacles.insert(obstacle);
        new_field
    }

    fn move_guard(&self, guard: &Guard) -> Option<Guard> {
        let turn_right = Complex::new(0, 1);
        let new_position = guard.pos + guard.dir;
        if self.obstacles.contains(&new_position) {
            return Some(Guard {
                pos: guard.pos,
                dir: guard.dir * turn_right,
            });
        }
        (self.width.contains(&new_position.re) && self.height.contains(&new_position.im)).then_some(
            Guard {
                pos: new_position,
                dir: guard.dir,
            },
        )
    }
}

fn part1(field: &Field) -> isize {
    let mut guard = Guard {
        pos: field.start,
        dir: Point::new(0, -1)
    };
    let mut seen: HashSet<Point> = HashSet::from([guard.pos]);
    while let Some(new_guard) = field.move_guard(&guard) {
        guard = new_guard;
        seen.insert(guard.pos);
    }
    seen.len() as isize
}

fn part2(field: &Field) -> isize {
    field.space.iter().filter(|&x| {
        let new_field = field.with_obstacle(*x);
        let mut guard = Guard {
            pos: field.start,
            dir: Point::new(0, -1)
        };
        let mut collisions: HashSet<(Point, Point)> = HashSet::new();
        while let Some(new_guard) = new_field.move_guard(&guard)  {
            if guard.pos == new_guard.pos {
                if!collisions.insert((new_guard.pos, new_guard.dir)) {
                    return true;
                }
            }
            guard = new_guard;
        }
        false

    }).count() as isize
}




fn main() {
    let input = load_input(2024, 6, None);
    let field = parse_input(&input);
    println!("Part 1: {:?}", part1(&field));
    println!("Part 2: {:?}", part2(&field));
}
