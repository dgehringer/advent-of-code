
use std::collections::{HashSet, VecDeque};

pub type Deck = VecDeque<usize>;
pub type History = HashSet<(Deck, Deck)>;

fn parse_input(content: &str) -> (Deck, Deck) {
    println!("{:?}", content);
    let parsed: Vec<Deck> = content
        .split("\n\n")
        .map(|player_block|
            player_block
                .lines()
                .filter_map(|l| l.parse::<usize>().ok()).collect())
        .collect();
    assert!(parsed.len() == 2);
    (parsed[0].clone(), parsed[1].clone())
}

macro_rules! push_cards {
    ($cond:expr, $d1:ident, $d2:ident, $c1:ident, $c2:ident) => {
        if $cond {
            $d1.push_back($c1);
            $d1.push_back($c2);
        } else {
            $d2.push_back($c2);
            $d2.push_back($c1);
        }
    };
}

fn play_round_part_one(d1: &mut Deck, d2: &mut Deck) {
    let (c1, c2) = (d1.pop_front().unwrap(), d2.pop_front().unwrap());
    push_cards!(c1 > c2, d1, d2, c1, c2);
}

fn part_one<'a>(d1: Deck, d2: Deck) -> (usize, Deck) {
    let mut counter: usize = 0;
    let (mut deck1, mut deck2) = (d1, d2);
    while deck1.len() > 0 && deck2.len() > 0 {
        play_round_part_one(&mut deck1, &mut deck2);
        counter += 1;
    }
    if deck1.len() > 0 { (1, deck1) } else { (2, deck2) }
}

fn part_two<'a>(d1: Deck, d2: Deck) -> (usize, Deck) {
    let (mut deck1, mut deck2) = (d1, d2);
    let mut history: History = History::new();
    while deck1.len() > 0 && deck2.len() > 0 {
        let current_round = (deck1.clone(), deck2.clone());
        if history.contains(&current_round) {
            return (1, deck1)
        }
        history.insert(current_round);
        let (c1, c2) = (deck1.pop_front().unwrap(), deck2.pop_front().unwrap());

        if deck1.len() >= c1 && deck2.len() >= c2 {
            let nd1: Deck = deck1.iter().map(|&f| f).take(c1).collect();
            let nd2: Deck = deck2.iter().map(|&f| f).take(c2).collect();
            let (winner, winning_deck) = part_two(nd1, nd2);

            push_cards!(winner == 1, deck1, deck2, c1, c2);
        } else {
            push_cards!(c1 > c2, deck1, deck2, c1, c2);
        }
    }
    if deck1.len() > 0 { (1, deck1) } else { (2, deck2) }
}

fn compute_score(d: &Deck) -> usize {
    d
        .iter()
        .zip((1..(d.len()+1)).rev())
        .map(|(&card, card_score)| card * card_score)
        .sum()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to open input");
    let (d1, d2) = parse_input(&contents);
    let (_, winner_deck) = part_one(d1.clone(), d2.clone());
    println!("Part1: {:?} {:?}", winner_deck, compute_score(&winner_deck));

    let (_, winner_deck) = part_two(d1.clone(), d2.clone());
    println!("Part2: {:?} {:?}", winner_deck, compute_score(&winner_deck));
}
