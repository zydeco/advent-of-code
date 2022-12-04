use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

/* short for Assignment */
type Ass = RangeInclusive<u8>;
type AssPair = (Ass, Ass);

fn read_line(s: String) -> AssPair {
    let parts: Vec<u8> = s
        .split(|c| c == '-' || c == ',')
        .into_iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    (parts[0]..=parts[1], parts[2]..=parts[3])
}

fn is_contained(p: &AssPair) -> bool {
    (p.0.start() >= p.1.start() && p.0.end() <= p.1.end())
        || (p.1.start() >= p.0.start() && p.1.end() <= p.0.end())
}

fn overlaps(p: &AssPair) -> bool {
    p.0.contains(p.1.start())
        || p.0.contains(p.1.end())
        || p.1.contains(p.0.start())
        || p.1.contains(p.0.end())
}

fn read_input() -> Vec<AssPair> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(read_line)
        .collect()
}

fn main() {
    let input = read_input();
    let contained = input.iter().filter(|ass| is_contained(ass)).count();
    println!("part1 {}", contained);

    let overlapping = input.iter().filter(|ass| overlaps(ass)).count();
    println!("part2 {}", overlapping);
}
