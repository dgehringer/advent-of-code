use std::collections::HashSet;

type Instructions = Vec<(char, i32)>;
type Point = (i32, i32);
type Rope = Vec<Point>;

fn parse_input(inp: &str) -> Instructions {
    inp.lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let dir = parts.next().unwrap().chars().next().unwrap();
            let amount: i32 = parts.next().unwrap().parse().unwrap();
            (dir, amount)
        })
        .collect()
}

fn modify<T>(f: T, a: Point, b: Point) -> Point
where
    T: Fn(i32, i32) -> i32,
{
    let (ax, ay) = a;
    let (bx, by) = b;
    (f(ax, bx), f(ay, by))
}

fn add(a: Point, b: Point) -> Point {
    modify(std::ops::Add::<i32>::add, a, b)
}

fn diff(a: Point, b: Point) -> Point {
    modify(std::ops::Sub::<i32>::sub, a, b)
}

fn clamp(a: i32) -> i32 {
    if a == 0 {
        0
    } else {
        a.signum()
    }
}

fn simulate(instructions: &Instructions, length: usize) -> HashSet<Point> {
    if length < 2 {
        panic!("A rope must at least have a head and a tail (length > 2)")
    }
    let mut rope: Rope = (0..length).map(|_| (0, 0)).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(*rope.last().unwrap());

    for &(direction, amount) in instructions {
        let vec: Point = match direction {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("Unknown direction {}", direction),
        };
        for _ in 0..amount {
            rope[0] = add(rope[0], vec);
            for i in 0..(rope.len() - 1) {
                let (dx, dy) = diff(rope[i], rope[i + 1]);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i + 1] = add(rope[i + 1], (clamp(dx), clamp(dy)));
                    if i == length - 2 {
                        visited.insert(rope[i + 1]);
                    }
                }
            }
        }
    }
    visited
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let instructions = parse_input(&contents);
    println!("Part 1: {:?}", simulate(&instructions, 2).len());
    println!("Part 2: {:?}", simulate(&instructions, 10).len());
}
