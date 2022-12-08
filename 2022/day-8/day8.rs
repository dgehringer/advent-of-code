
type Board = Vec<Vec<u8>>;

fn parse_input(inp: &str) -> Board {
    inp.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn shape(board: &Board) -> (usize, usize) {
    (board.len(), board.get(0).unwrap().len())
}

fn generate_axes(
    i: usize,
    j: usize,
    board: &Board,
) -> (
    impl Iterator<Item = u8> + '_ + Clone,
    impl Iterator<Item = u8> + '_ + Clone,
    impl Iterator<Item = u8> + '_ + Clone,
    impl Iterator<Item = u8> + '_ + Clone,
) {
    let (nx, ny) = shape(board);
    let left = (0..j).rev().map(move |jj| board[i][jj]);
    let right = ((j + 1)..nx).map(move |jj| board[i][jj]);
    let up = (0..i).rev().map(move |ii| board[ii][j]);
    let down = ((i + 1)..ny).map(move |ii| board[ii][j]);
    (left, right, up, down)
}

fn visible_trees(board: &Board) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (nx, ny) = shape(board);
    (0..nx)
        .flat_map(move |i| (0..ny).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            let height = board[i][j];
            let lower = |h| h < height;
            let (mut l, mut r, mut u, mut d) = generate_axes(i, j, board);
            l.all(lower) || r.all(lower) || u.all(lower) || d.all(lower)
        })
}

fn part_one(board: &Board) {
    println!("Part 1: {}", visible_trees(board).count());
}

fn score(ax: impl Iterator<Item = u8> + Clone, height: u8) -> usize {
    let n = ax.clone().count();
    if n == 0 {
        return 0;
    }
    let mut score = 1;
    for other_height in ax.take(n - 1) {
        if height > other_height {
            score += 1;
        } else {
            break;
        }
    }
    score
}

fn part_two(board: &Board) {
    let best_score = visible_trees(board)
        .map(|(i, j)| {
            let h = board[i][j];
            let (l, r, u, d) = generate_axes(i, j, board);
            score(l, h) * score(r, h) * score(u, h) * score(d, h)
        })
        .max();
    println!("Part 2: {}", best_score.unwrap());
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let board = parse_input(&contents);
    part_one(&board);
    part_two(&board);
}
