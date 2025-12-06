
mod input;


type Instruction = (char, isize);

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (char, rest) = line.split_at(1);
            (char.chars().take(1).next().unwrap(), rest.trim().parse::<isize>().unwrap())
        }).collect()
}

fn part1(instructions: &[Instruction]) -> isize {
    let mut clicks = 0isize;
    let mut dial = 50isize;
    for (char, amount) in instructions {
        dial = match char {
            'L' => (dial - amount),
            'R' => (dial + amount),
            _ => unimplemented!(),
        } % 100;
        if dial < 0 {
            dial += 100;
        }
        if dial == 0 {
            clicks += 1;
        }
    }
    clicks
}

fn part2(instructions: &[Instruction]) -> isize {
    let mut clicks = 0isize;
    let dial = 50isize;
    for (char, amount) in instructions {
        clicks += match char {
            'L' => dial+1..=dial+amount,
            'R' => dial-1..=dial-amount,
            _ => unimplemented!(),
        }.filter(|d| d % 100 == 0).count() as isize
    }
    clicks * 2 - 1
}

fn main() {
    let input = load_input(2025, 1, None);
    let instructions = parse_input(&input);
    println!("Part 1: {:?}", part1(&instructions));
    println!("Part 2: {:?}", part2(&instructions));

}