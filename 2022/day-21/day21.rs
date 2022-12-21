use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Task<'a> {
    Calculate(char, &'a str, &'a str),
    Yell(i64),
}

type Instructions<'a> = HashMap<&'a str, Task<'a>>;

fn parse_task(content: &str) -> Option<Task> {
    let parts: Vec<&str> = content.split(' ').collect();
    match parts.len() {
        1 => Some(Task::Yell(parts[0].parse::<i64>().unwrap())),
        3 => Some(Task::Calculate(
            parts[1].chars().next().unwrap(),
            parts[0],
            parts[2],
        )),
        _ => None,
    }
}

fn parse_input(contents: &str) -> Instructions {
    contents
        .lines()
        .map(|line| {
            let mut sep = line.split(": ");
            let name = sep.next().unwrap();
            let task = parse_task(sep.next().unwrap()).unwrap();
            (name, task)
        })
        .collect()
}

fn solve(name: &str, instructions: &Instructions) -> i64 {
    match instructions[name] {
        Task::Yell(n) => n,
        Task::Calculate(op, l, r) => {
            let left = solve(l, instructions);
            let right = solve(r, instructions);
            match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => unreachable!(),
            }
        }
    }
}

fn contains_expr(what: &str, name: &str, instructions: &Instructions) -> bool {
    if what == name {
        return true;
    }
    match instructions[name] {
        Task::Yell(_) => false,
        Task::Calculate(_, l, r) => {
            contains_expr(what, l, instructions) || contains_expr(what, r, instructions)
        }
    }
}

fn solve_for(what: &str, name: &str, equal: i64, instructions: &Instructions) -> i64 {
    match instructions[name] {
        Task::Yell(_) => unreachable!(),
        Task::Calculate(op, l, r) => {
            let solve_for_left = contains_expr(what, l, instructions);
            let (to_solve, value) = if solve_for_left {
                (l, solve(r, instructions))
            } else {
                (r, solve(l, instructions))
            };
            let new_equal = match op {
                '+' => equal - value,
                '*' => equal / value,
                '-' => {
                    if solve_for_left {
                        equal + value
                    } else {
                        value - equal
                    }
                }
                '/' => {
                    if solve_for_left {
                        equal * value
                    } else {
                        value / equal
                    }
                }
                _ => unreachable!(),
            };
            if what == to_solve {
                new_equal
            } else {
                solve_for(what, to_solve, new_equal, instructions)
            }
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let instructions = parse_input(&contents);
    println!("Part 1: {}", solve("root", &instructions));
    if let Task::Calculate(_, left, right) = &instructions["root"] {
        let mut mod_inst = instructions.clone();
        *mod_inst.get_mut("root").unwrap() = Task::Calculate('-', left, right);
        println!(
            "Part 2: {}",
            solve_for("humn", "root", 0, &mod_inst)
        );
    }
}
