use std::collections::HashSet;

type Point = (i32, i32);
type Grid = Vec<Vec<char>>;

fn parse_input(contents: &str) -> Grid {
    let h = contents.lines().count();
    let w = contents.lines().next().unwrap().chars().count();
    contents.lines().skip(1).take(h-2).map(|l| l.chars().skip(1).take(w-2).collect()).collect()
}

fn size(grid: &Grid) -> Point {
    (grid.len() as i32, grid[0].len() as i32)
}

fn wrap(v: i32, l: i32) -> usize {
    let w = v % l;
    if w >= 0 { w as usize }
    else { (l + w) as usize}
}

fn simulate_blizzards(basin: &Grid, start: &Point, stop: &Point, start_time: i32) -> i32{
    let (height, width) = size(basin);
    let mut p = HashSet::from([*start]); // current blizzard position
    let mut time = start_time;
    loop {
        let mut np = HashSet::<Point>::new();
        for (y, x) in p { // compute next blizzard state
            // check all neighbors for each blizzard
            for (ny, nx) in [(y, x), (y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if (ny, nx) == *stop { return time;}
                if 0 <= ny
                    && ny < height
                    && 0 <= nx
                    && nx < width
                   && basin[ny as usize][wrap(nx - time, width)] != '>'
                   && basin[ny as usize][wrap(nx + time, width)] != '<'
                   && basin[wrap(ny - time, height)][nx as usize] != 'v'
                   && basin[wrap(ny + time, height)][nx as usize] != '^' {
                    np.insert((ny, nx));
                }
            }
        }
        p = np;
        if p.is_empty(){ p.insert(*start); }
        time += 1;
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let grid = parse_input(&contents);
    let (h, w) = size(&grid);
    let (start, end) = ((-1, 0), (h, w -1));
    let first = simulate_blizzards(&grid, &start, &end, 1);
    println!("Part one: {}", first);
    println!("Part two: {}", simulate_blizzards(&grid, &start, &end, simulate_blizzards(&grid, &start, &end, first)));
}

