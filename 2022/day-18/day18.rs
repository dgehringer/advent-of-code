use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};

type Cube = (i32, i32, i32);

fn parse_input(inp: &str) -> Vec<Cube> {
    inp.lines()
        .map(|line| {
            line.split(',')
                .map(|coord| coord.parse::<i32>().expect("Failed to parse coord"))
                .collect::<Vec<i32>>()
        })
        .map(|cv| (cv[0], cv[1], cv[2]))
        .collect()
}

fn neighbors(cube: &Cube) -> [Cube; 6] {
    let (x, y, z) = *cube;
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn part_one(cubes: &[Cube]) {
    let result = cubes
        .iter()
        .map(|c| {
            neighbors(c)
                .iter()
                .map(|n| !cubes.contains(n) as i32)
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("Part 1: {}", result);
}

fn part_two(cubes: &[Cube]) {
    let (minx, miny, minz, maxx, maxy, maxz) = cubes.iter().cloned().fold(
        (i32::MAX, i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN),
        |a, b| {
            (
                min(a.0, b.0),
                min(a.1, b.1),
                min(a.2, b.2),
                max(a.3, b.0),
                max(a.4, b.1),
                max(a.5, b.2),
            )
        },
    );
    let in_bounds: Box<dyn Fn(Cube) -> bool> = Box::new(|c| {
        let (x, y, z) = c;
        minx - 1 <= x
            && x <= maxx + 1
            && miny - 1 <= y
            && y <= maxy + 1
            && minz - 1 <= z
            && z <= maxz + 1
    });
    let mut visited = HashSet::<Cube>::from_iter(cubes.iter().cloned());
    let mut queue: VecDeque<Cube> = VecDeque::from([(minx, miny, minz)]);
    let mut reachable_tiles = 0_i32;
    while !queue.is_empty() {
        for cube in neighbors(&queue.pop_front().unwrap()) {
            if !in_bounds(cube) {
                continue;
            }
            if cubes.contains(&cube) {
                reachable_tiles += 1;
            }
            if visited.contains(&cube) {
                continue;
            }
            visited.insert(cube);
            queue.push_back(cube);
        }
    }
    println!("Part 2: {:?}", reachable_tiles);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let cubes = parse_input(&contents);
    part_one(&cubes);
    part_two(&cubes);
}
