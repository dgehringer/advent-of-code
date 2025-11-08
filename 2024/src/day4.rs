mod input;

use crate::input::load_input;

#[derive(Debug)]
struct Field {
    data: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Field {
    fn at(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.data
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }
    fn coords(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.w).flat_map(move |x| (0..self.h).map(move |y| (x as i32, y as i32)))
    }

    fn is_char(&self, x: i32, y: i32, c: char) -> bool {
        match self.at(x, y) {
            Some(ch) => ch == c,
            _ => false,
        }
    }
}

fn parse_input(input: &str) -> Field {
    let data = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let h = data.len();
    let w = data[0].len();
    Field { data, w, h }
}

fn xmas_in_direction(field: &Field, start: (i32, i32), direction: (i32, i32)) -> usize {
    let (x0, y0) = start;
    let (dx, dy) = direction;
    if "XMAS"
        .chars()
        .enumerate()
        .all(|(i, c)| field.is_char(x0 + dx * (i as i32), y0 + dy * (i as i32), c))
    {
        1
    } else {
        0
    }
}

fn part1(field: &Field) -> usize {
    let directions = [
        (1, 0),   // right
        (0, 1),   // down
        (1, 1),   // down-right
        (1, -1),  // up-right
        (-1, 0),  // left
        (0, -1),  // up
        (-1, -1), // up-left
        (-1, 1),  // down-left
    ];
    field
        .coords()
        .flat_map(|(x, y)| directions.map(|direction| (x, y, direction)))
        .map(|(x, y, direction)| xmas_in_direction(field, (x, y), direction))
        .sum()
}

fn has_xmas(field: &Field, start: (i32, i32)) -> usize {
    let (x0, y0) = start;
    let is_char_relative = |dx: i32, dy: i32, c: char| field.is_char(x0 + dx, y0 + dy, c);
    let opposite_is_m_and_s = |dx: i32, dy: i32| {
        (is_char_relative(dx, dy, 'M') && is_char_relative(-dx, -dy, 'S'))
            || (is_char_relative(dx, dy, 'S') && is_char_relative(-dx, -dy, 'M'))
    };

    if is_char_relative(0, 0, 'A') && opposite_is_m_and_s(1, 1) && opposite_is_m_and_s(1, -1) {
        1
    } else {
        0
    }
}

fn part2(field: &Field) -> usize {
    field.coords().map(|coord| has_xmas(field, coord)).sum()
}

fn main() {
    let input = load_input(2024, 4, None);
    let field = parse_input(&input);
    println!("Part 1: {:?}", part1(&field));
    println!("Part 2: {:?}", part2(&field));
}
