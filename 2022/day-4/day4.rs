use std::collections::HashSet;

type Range = HashSet<u8>;
type Sections = Vec<(Range, Range)>;

fn parse_section(section: &str) -> Range {
    let mut bounds = section.split('-');
    let minb: u8 = bounds
        .next()
        .unwrap()
        .parse()
        .expect("Failed to parse lower bound");
    let maxb: u8 = bounds
        .next()
        .unwrap()
        .parse()
        .expect("Failed to parse upper bound");
    (minb..(maxb + 1)).collect()
}

fn parse_input(contents: &str) -> Sections {
    contents
        .lines()
        .map(|l| {
            let mut sections = l.split(',');
            let s1 = sections.next().unwrap();
            let s2 = sections.next().unwrap();
            (parse_section(s1), parse_section(s2))
        })
        .collect()
}

fn part_one(sections: &Sections) {
    let num_contained: usize = sections
        .iter()
        .map(|(s1, s2)| (s1.is_subset(s2) || s2.is_subset(s1)) as usize)
        .sum();
    println!("Part 1: {:?}", num_contained);
}

fn part_two(sections: &Sections) {
    let num_overlap: usize = sections
        .iter()
        .map(|(s1, s2)| (s1.intersection(s2).count() > 0) as usize)
        .sum();
    println!("Part 2: {:?}", num_overlap);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let sections = parse_input(&contents);
    part_one(&sections);
    part_two(&sections);
}
