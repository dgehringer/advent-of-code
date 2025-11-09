mod input;

use crate::input::load_input;
use itertools::Itertools;
use std::collections::HashSet;

type Rule = (i32, i32);
type Rules = HashSet<Rule>;
type Update = Vec<i32>;

fn parse_input(input: &str) -> (Rules, Vec<Update>) {
    input
        .split("\n\n")
        .collect_tuple()
        .map(|(rules_str, updates_str)| {
            let page_regex = regex::Regex::new(r"\d+").unwrap();
            let rules: Rules = rules_str
                .lines()
                .filter_map(|line| {
                    line.trim()
                        .split("|")
                        .tuple_windows()
                        .next()
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                })
                .collect();
            let updates: Vec<Update> = updates_str
                .lines()
                .filter_map(|line| {
                    let update: Update = page_regex
                        .captures_iter(line)
                        .map(|c| c[0].parse::<i32>().unwrap())
                        .collect();
                    if !update.is_empty() {
                        Some(update)
                    } else {
                        None
                    }
                })
                .collect();
            (rules, updates)
        })
        .unwrap()
}

fn in_right_order(rules: &Rules, update: &Update) -> Option<i32> {
    if update
        .iter()
        .tuple_windows()
        .all(|(p1, p2)| rules.contains(&(*p1, *p2)))
    {
        Some(update[update.len() / 2])
    } else {
        None
    }
}

fn part1(rules: &Rules, updates: &[Update]) -> i32 {
    updates
        .iter()
        .filter_map(|update| in_right_order(rules, update))
        .sum()
}

fn fix_update(rules: &Rules, update: &Update) -> Option<Update> {
    for (i, (p1, p2)) in update.iter().tuple_windows().enumerate() {
        if !rules.contains(&(*p1, *p2)) {
            if rules.contains(&(*p2, *p1)) {
                // we can fix it by swapping
                let mut fixed = update.clone();
                fixed.swap(i, i + 1);
                return fix_update(rules, &fixed);
            } else {
                // this update cannot be fixed any more
                return None;
            }
        }
    }
    Some(update.clone())
}
fn part2(rules: &Rules, updates: &[Update]) -> i32 {
    updates
        .iter()
        .filter(|update| in_right_order(rules, update).is_none())
        .filter_map(|update| fix_update(rules, update))
        .filter_map(|update| in_right_order(rules, &update))
        .sum()
}

fn main() {
    let input = load_input(2024, 5, None);
    let (rules, upates) = parse_input(&input);
    println!("Part 1: {:?}", part1(&rules, &upates));
    println!("Part 2: {:?}", part2(&rules, &upates));
}
