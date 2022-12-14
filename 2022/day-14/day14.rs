use std::collections::HashSet;

type Point = (i32, i32);
type Rocks = HashSet<Point>;
type Board = (Rocks, Point, Point);

fn point_op<T: Clone>(a: &(T, T), b: &(T, T), op: impl Fn(T, T) -> T) -> (T, T) {
    let (ar, ad) = a.clone();
    let (br, bd) = b.clone();
    (op(ar, br), op(ad, bd))
}

fn add(a: &Point, b: &Point) -> Point {
    point_op(a, b, std::ops::Add::add)
}

fn sub(a: &Point, b: &Point) -> Point {
    point_op(a, b, std::ops::Sub::sub)
}

fn unit_vector(a: Point) -> Point {
    (
        if a.0 != 0 { a.0.signum() } else { 0 },
        if a.1 != 0 { a.1.signum() } else { 0 },
    )
}

fn make_line(start: &Point, end: &Point) -> Rocks {
    let mut rocks: Rocks = HashSet::from_iter(vec![start.clone(), end.clone()]);
    let diff = unit_vector(sub(end, start));
    let mut p = *start;
    while &p != end {
        p = add(&p, &diff);
        rocks.insert(p.clone());
    }
    rocks
}

fn parse_input(contents: &str, part_two: bool) -> Board {
    let lines: Vec<Vec<Point>> = contents
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coords| {
                    let mut sep = coords.split(',');
                    (
                        sep.next().unwrap().parse().unwrap(),
                        sep.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    let mut rocks = Rocks::new();
    lines.iter().for_each(|line| {
        line.iter()
            .take(line.len() - 1)
            .zip(line.iter().skip(1))
            .for_each(|(start, end)| {
                make_line(start, end).iter().for_each(|&p| {
                    rocks.insert(p);
                })
            })
    });
    let mut max_down = rocks.iter().map(|(_, d)| *d).max().unwrap();

    if part_two {
        max_down += 2;
        // bottom line does not need to be 2 * max_down
        make_line(
            &(500 - max_down - 1, max_down),
            &(500 + max_down + 1, max_down),
        )
        .iter()
        .for_each(|&p| {
            rocks.insert(p);
        });
    }
    let max_right = rocks.iter().map(|(r, _)| *r).max().unwrap();
    let min_point = (rocks.iter().map(|(r, _)| *r).min().unwrap(), 0);

    (rocks, min_point, (max_right, max_down))
}

fn fell_into_abyss(board: &Board, _: &Point, new_pos: Option<Point>) -> bool {
    match new_pos {
        Some((_, pd)) => {
            let (_, _, (_, ld)) = board;
            pd > *ld
        }
        None => false,
    }
}

fn source_reached(_: &Board, grain: &Point, new_pos: Option<Point>) -> bool {
    grain == &(500, 0) && new_pos.is_none()
}

fn can_move(gr: &Point, rocks: &Rocks, sand: &Rocks) -> Option<Point> {
    for direction in &[(0, 1), (-1, 1), (1, 1)] {
        let new_pos = add(&gr, direction);
        if !rocks.contains(&new_pos) && !sand.contains(&new_pos) {
            return Some(new_pos);
        }
    }
    None
}

fn simulate(
    board: &Board,
    source: &Point,
    finished: impl Fn(&Board, &Point, Option<Point>) -> bool,
) -> Rocks {
    let (rocks, _, _) = board;
    let mut sand = Rocks::new();

    let mut can_add_sand = true;
    while can_add_sand {
        let mut grain = source.clone();
        let mut p = can_move(&grain, rocks, &sand);
        can_add_sand = !finished(board, &grain, p);
        while p.is_some() {
            grain = p.unwrap();
            p = can_move(&grain, rocks, &sand);
            can_add_sand = !finished(board, &grain, p);
            if !can_add_sand {
                break;
            }
        }
        if can_add_sand {
            sand.insert(grain.clone());
        } else {
            break;
        }
    }
    return sand;
}

fn draw_board(board: &Board) {
    let (rocks, minp, maxp) = board;
    for y in minp.1..=maxp.1 {
        for x in minp.0..=maxp.0 {
            let mut ch = if rocks.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", ch);
        }
        println!("");
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let source = (500, 0);
    let board = parse_input(&contents, false);
    println!(
        "Part 1: {}",
        simulate(&board, &source, fell_into_abyss).len()
    );
    let board = parse_input(&contents, true);
    println!(
        "Part 2: {}",
        simulate(&board, &source, source_reached).len()
    );
}
