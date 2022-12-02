type Strategy = Vec<(char, char)>;

fn parse_input(content: &str) -> Strategy {
    content
        .split('\n')
        .map(|l| {
            let mut sep = l.split(' ');
            let c1 = sep.next().unwrap().chars().next().unwrap();
            let c2 = sep.next().unwrap().chars().next().unwrap();
            assert!("ABC".contains(c1));
            assert!("XYZ".contains(c2));
            (c1, c2)
        })
        .collect()
}

fn game_score(elf: char, me: char, choose_me: fn(char, char) -> char) -> i32 {
    let me = choose_me(elf, me);
    let choice_score = me as i32 - 'W' as i32;
    let diff = me as i32 - elf as i32;
    match diff {
        23 => choice_score + 3,
        24 => choice_score + 6,
        21 => choice_score + 6,
        22 => choice_score,
        25 => choice_score,
        _ => panic!("Unreachable"),
    }
}

fn part_one(strategy: &Strategy) {
    let sum = strategy
        .iter()
        .map(|&(elf, me)| game_score(elf, me, |_, me| me))
        .sum::<i32>();
    println!("Part 1: {:?}", sum);
}

fn choose_me(elf: char, me: char) -> char {
    let elf_msg = "Elf can only choose between A, B and C";
    match me {
        'Y' => (elf as u8 + 23u8) as char,
        'X' => match elf {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => panic!("{}", elf_msg),
        },
        'Z' => match elf {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => panic!("{}", elf_msg),
        },
        _ => panic!("Unreachable"),
    }
}

fn part_two(strategy: &Strategy) {
    let sum = strategy
        .iter()
        .map(|&(elf, me)| game_score(elf, me, choose_me))
        .sum::<i32>();
    println!("Part 2: {:?}", sum);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let strategy = parse_input(&contents);
    part_one(&strategy);
    part_two(&strategy);
}
