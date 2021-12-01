use std::fs::read_to_string;
use std::path::Path;
use regex::Regex;

type Person = std::collections::HashSet<char>;
type Group = Vec<Person>;

fn main() {
    let contents = read_to_string(Path::new("/media/dominik/DATEN/drive/projects/advent-of-code-2020/day-6/input.txt")).unwrap();

    let group_regex = Regex::new(r"\s{2,}").unwrap();

    let groups : Vec<Group> = group_regex.split(contents.as_str())
        .map(|group| {
            group.lines()
                .map(|person| person.chars().collect::<Person>()).collect()
        }).collect();

    
    let sum1 :usize = groups
        .iter()
        .map(|group|{
            group
                .iter()
                .fold(Person::new(), |mut merged, p| {
                    merged.extend(p);
                    merged
                } )
        })
        .map(|group| group.len())
        .sum();

    let sum2: usize  = groups
        .iter()
        .map(|group|{
            group
                .iter()
                .fold(group.first().unwrap().clone() ,|merged, p| {
                    merged.intersection(p).copied().collect()
                } )
        })
        .map(|group| group.len())
        .sum();
    
    println!("{:?}, {:?}", sum1, sum2);
}
