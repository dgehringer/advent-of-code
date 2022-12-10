
fn run_program(program: &str) {
    let mut register = 1_i32;
    let mut register_history: Vec<i32> = Vec::new();

    program.lines().for_each(|l| {
        if l.starts_with("noop") {
            register_history.push(register);
        } else if l.starts_with("addx") {
            let diff = l
                .split(' ')
                .take(2)
                .last()
                .unwrap()
                .parse::<i32>()
                .expect("Failed to parse number");
            register_history.push(register);
            register_history.push(register);
            register += diff;
        }
    });

    let signal_strength: i32 = register_history
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(clock, &reg)| ((clock + 1) as i32) * reg)
        .sum();
    println!("Part 1: {:?}", signal_strength);

    let display_width = 40_usize;
    let display: Vec<char> = register_history
        .iter()
        .enumerate()
        .map(|(i, &reg)| match (i % display_width) as i32 - reg {
            (-1..=1) => '#',
            _ => '.',
        })
        .collect();

    println!("Part: 2");
    for line in display[..].chunks(40) {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    run_program(&contents);
}
