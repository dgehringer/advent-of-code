use regex::Regex;
use std::cmp::max;
use rayon::prelude::*;
use std::collections::HashMap;

type Blueprint = (i32, i32, i32, i32, i32, i32);
type State = Blueprint;

fn parse_input(inp: &str) -> Vec<Blueprint> {
    let num_regex = Regex::new(r"(\d+)").unwrap();
    inp.lines()
        .map(|line| {
            let nums: Vec<i32> = num_regex
                .find_iter(line)
                .map(|m| line[m.start()..m.end()].parse::<i32>().unwrap())
                .collect();
            (nums[1], nums[2], nums[3], nums[4], nums[5], nums[6])
        })
        .collect()
}

fn update_states(states: &mut HashMap<State, i32>, state: State, geodes: i32) {
    if states.contains_key(&state) {
        if let Some(g_curr) = states.get_mut(&state) {
            *g_curr = max(geodes, *g_curr);
        }
    } else {
        states.insert(state, geodes);
    }
}

fn max_num_geodes(blueprint: Blueprint, minutes: i32) -> i32 {
    let (p_o_o, p_c_o, p_b_o, p_b_c, p_g_o, p_g_b) = blueprint;
    //Meaning of state is (ore, clay, obsidian, robots_ore, robots_clay, robots_obsidian)
    let initial_state = (0, 0, 0, 1, 0, 0);
    let mut states: HashMap<State, i32> = HashMap::from_iter([(initial_state, 0)]);
    let max_o = *[p_o_o, p_c_o, p_b_o, p_g_o].iter().max().unwrap();
    for minute in (2..=minutes).rev() {
        let mut current_states: HashMap<State, i32> = HashMap::new();
        for (state, geodes) in &states {
            let (o, c, b, r_o, r_c, r_b) = *state;
            let build_ore = o >= p_o_o && r_o < max_o && o + (r_o * minute) < max_o * minute;
            let build_clay = o >= p_c_o && r_c < p_b_c && c + (r_c * minute) < p_b_c * minute;
            let build_obsidian = o >= p_b_o && c >= p_b_c && r_b < p_g_b;
            let build_geode = o >= p_g_o && b >= p_g_b;

            // there's no point in in building a helper robot less the 2 min before time is up
            if minute >= 2 {
                if build_ore {
                    update_states(
                        &mut current_states,
                        (o + r_o - p_o_o, c + r_c, b + r_b, r_o + 1, r_c, r_b),
                        *geodes,
                    );
                }
                if build_clay {
                    update_states(
                        &mut current_states,
                        (o + r_o - p_c_o, c + r_c, b + r_b, r_o, r_c + 1, r_b),
                        *geodes,
                    );
                }
                if build_obsidian {
                    update_states(
                        &mut current_states,
                        (o + r_o - p_b_o, c + r_c - p_b_c, b + r_b, r_o, r_c, r_b + 1),
                        *geodes,
                    );
                }
                if !(build_ore && build_clay && build_obsidian && build_geode) {
                    update_states(
                        &mut current_states,
                        (o + r_o, c + r_c, b + r_b, r_o, r_c, r_b),
                        *geodes,
                    );
                }
            }
            if build_geode {
                update_states(
                    &mut current_states,
                    (o + r_o - p_g_o, c + r_c, b + r_b - p_g_b, r_o, r_c, r_b),
                    geodes + (minute - 1),
                );
            }
        }
        states = current_states;
    }
    states.values().cloned().max().unwrap_or(0)
}

fn part_one(blueprints: &[Blueprint]) {
    let solution: i32 = blueprints
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(i, bp)| (i as i32 + 1) * max_num_geodes(*bp, 24))
        .sum();
    println!("Part 1: {}", solution);
}

fn part_two(blueprints: &[Blueprint]) {
    let first_three: Vec<i32> = blueprints
        .iter()
        .take(3)
        .par_bridge()
        .map(|bp| max_num_geodes(*bp, 32))
        .collect();
    println!(
        "Part 2: {:?}",
        first_three[0] * first_three[1] * first_three[2]
    );
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let bp = parse_input(&contents);
    part_one(&bp);
    part_two(&bp);
}
