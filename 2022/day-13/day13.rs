use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Value(usize),
    Packet(Vec<Packet>),
}

fn consume_tokens(tokens: &mut Vec<&str>) -> Vec<Packet> {
    let mut r: Vec<Packet> = Vec::new();
    while !tokens.is_empty() {
        match tokens.pop() {
            Some("") => {}
            Some("]") => {
                return r;
            }
            Some("[") => r.push(Packet::Packet(consume_tokens(tokens))),
            Some(n) => r.push(Packet::Value(n.parse::<usize>().unwrap())),
            None => unreachable!(),
        }
    }
    r
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.replace('[', "[,").replace(']', ",]");
        let mut tokens = binding.split(',').rev().collect();
        Ok(Packet::Packet(consume_tokens(&mut tokens)))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(l), Packet::Value(r)) => l.cmp(r),
            (Packet::Value(l), Packet::Packet(_)) => {
                Packet::Packet(vec![Packet::Value(*l)]).cmp(other)
            }
            (Packet::Packet(_), Packet::Value(r)) => {
                self.cmp(&Packet::Packet(vec![Packet::Value(*r)]))
            }
            (Packet::Packet(left), Packet::Packet(right)) => {
                for i in 0..(left.len().min(right.len())) {
                    match left[i].cmp(&right[i]) {
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

fn parse_input(inp: &str) -> Vec<(Packet, Packet)> {
    inp.split("\n\n")
        .map(|packet_pair| {
            let mut lines = packet_pair.lines();
            (
                lines.next().unwrap().parse::<Packet>().unwrap(),
                lines.next().unwrap().parse::<Packet>().unwrap(),
            )
        })
        .collect()
}

fn part_one(packet_pairs: &[(Packet, Packet)]) {
    let pair_index_sum: usize = packet_pairs
        .iter()
        .enumerate()
        .map(|(i, (left, right))| if left < right { i + 1 } else { 0 })
        .sum();
    println!("Part 1: {}", pair_index_sum);
}

fn find_packet(packets: &[Packet], packet: &Packet) -> usize {
    packets.iter().position(|p| p == packet).unwrap()
}

fn part_two(packet_pairs: &[(Packet, Packet)]) {
    let mut all_packets: Vec<Packet> = Vec::new();
    packet_pairs.iter().for_each(|(l, r)| {
        all_packets.push(l.clone());
        all_packets.push(r.clone());
    });
    let first_div = "[[2]]".parse::<Packet>().unwrap();
    let second_div = "[[6]]".parse::<Packet>().unwrap();
    all_packets.push(first_div.clone());
    all_packets.push(second_div.clone());
    all_packets.sort();
    println!(
        "Part 2: {}",
        (find_packet(&all_packets, &first_div) + 1) * (find_packet(&all_packets, &second_div) + 1)
    )
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let packets = parse_input(&contents);
    part_one(&packets);
    part_two(&packets);
}
