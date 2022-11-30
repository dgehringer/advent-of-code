use std::collections::{HashMap, HashSet};
use std::fs;

pub type Ingredient<'a> = &'a str;
pub type Allergene<'a> = &'a str;
pub type Mapping<'a> = (HashSet<Ingredient<'a>>, HashSet<Allergene<'a>>);

#[macro_export]
macro_rules! read_input {
    ($varname:ident, $fname:literal) => {
        let $varname = fs::read_to_string($fname).expect("Could not read puzzle input");
    };
}

fn parse_input(contents: &str) -> Vec<Mapping> {
    contents
        .lines()
        .map(|line| {
            let mut sep = line.split(" (contains ");
            let ingredients_raw = sep.next().unwrap();
            let allergenes_raw = sep.next().unwrap().strip_suffix(")").unwrap();
            (
                ingredients_raw.split(' ').collect(),
                allergenes_raw.split(", ").collect(),
            )
        })
        .collect()
}

fn allergene_to_food<'a>(foodlist: &'a Vec<Mapping<'a>>) -> HashMap<Allergene<'a>, Ingredient<'a>> {
    let mut possible_ingredients: HashMap<Allergene, HashSet<Ingredient>> = HashMap::new();
    for (food_ingredients, food_allergenes) in foodlist {
        for allergene in food_allergenes {
            if !possible_ingredients.contains_key(allergene) {
                possible_ingredients.insert(allergene, food_ingredients.clone());
            } else {
                *possible_ingredients.get_mut(allergene).unwrap() = possible_ingredients[allergene]
                    .intersection(food_ingredients)
                    .map(|&ing| ing)
                    .collect();
            }
        }
    }

    let mut solved: HashMap<Allergene, Ingredient> = HashMap::new();
    while possible_ingredients.len() > 0 {
        let (allergene, foods) = possible_ingredients
            .clone()
            .into_iter()
            .filter(|(_, ingredients)| ingredients.len() == 1)
            .next()
            .unwrap();
        let food = foods.iter().next().unwrap();
        solved.insert(allergene, food);
        possible_ingredients.remove(allergene);
        for (_, allergene_foods) in possible_ingredients.iter_mut() {
            if allergene_foods.contains(food) {
                allergene_foods.remove(food);
            }
        }
    }
    solved
}

fn part1<'a>(foodlist: &'a Vec<Mapping<'a>>) {
    let mut all_ingredients: Vec<Ingredient> =
        foodlist
            .iter()
            .map(|(ing, _)| ing)
            .fold(Vec::<Ingredient>::new(), |mut r, ing| {
                ing.iter().for_each(|ingredient| r.push(ingredient));
                r
            });

    let bad_ingredients: HashSet<Ingredient> =
        allergene_to_food(foodlist).values().map(|&f| f).collect();
    let num_ingredients: i32 = all_ingredients
        .iter()
        .map(|i| (!bad_ingredients.contains(i)) as i32)
        .sum();

    println!("Part1: {:?}", num_ingredients);
}

fn part2<'a>(foodlist: &'a Vec<Mapping<'a>>) {
    let solution = allergene_to_food(foodlist);
    let mut allergenes: Vec<Allergene> = solution.keys().map(|&a| a).collect();
    allergenes.sort();
    let ingredients: Vec<Ingredient> = allergenes
        .iter()
        .map(|allergene| *solution.get(allergene).unwrap())
        .collect();
    println!("Part2: {:?}", ingredients.join(","))
}

fn main() {
    read_input!(contents, "input.txt");
    let food = parse_input(contents.as_str());
    part1(&food);
    part2(&food);
}
