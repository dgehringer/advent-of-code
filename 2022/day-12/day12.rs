use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<u8>>;
type Point = (i32, i32);

fn parse_input(inp: &str) -> (Grid, Point, Point) {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let board: Grid = inp
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    let chr: char = match c {
                        'S' => {
                            start = Some((i as i32, j as i32));
                            'a'
                        }
                        'E' => {
                            end = Some((i as i32, j as i32));
                            'z'
                        }
                        'a'..='z' => c,
                        _ => panic!("Unknown elevation"),
                    };
                    ((chr as usize) - ('a' as usize)) as u8 + 1_u8
                })
                .collect()
        })
        .collect();
    (board, start.unwrap(), end.unwrap())
}

fn in_bounds(index: i32, bound: usize) -> bool {
    index >= 0 && index < bound as i32
}

fn find_path(grid: &Grid, start: Point, end: Point) -> Option<usize> {
    let (ny, nx) = (grid.len(), grid.get(0).unwrap().len());
    let mut seen: HashSet<Point> = HashSet::new();
    let mut q: VecDeque<(Point, usize)> = VecDeque::new();
    q.push_front((start, 0_usize));

    while q.len() != 0 {
        let (loc, cost) = q.pop_front().unwrap();
        if loc == end {
            return Some(cost);
        }
        if !seen.insert(loc) {
            continue;
        }
        for (dy, dx) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (y, x) = loc;
            let (py, px) = (y + dy, x + dx);
            if in_bounds(py, ny) && in_bounds(px, nx) {
                if grid[py as usize][px as usize] <= 1 + grid[y as usize][x as usize] {
                    q.push_back(((py, px), cost + 1))
                }
            }
        }
    }
    None
}

fn minimal_path(grid: &Grid, end: Point) -> usize {
    let (ny, nx) = (grid.len(), grid.get(0).unwrap().len());
    (0..ny)
        .flat_map(move |i| (0..nx).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 1)
        .filter_map(|(y, x)| find_path(grid, (y as i32, x as i32), end))
        .min()
        .unwrap()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let (grid, start, end) = parse_input(&contents);
    println!("Part 1: {:?}", find_path(&grid, start, end).unwrap());
    println!("Part 2: {:?}", minimal_path(&grid, end));
}
