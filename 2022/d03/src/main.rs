use std::{
    collections::HashSet,
    fmt::Debug,
    io::{self, BufRead},
};

struct Rucksack {
    items: Vec<u8>,
}

fn priority(val: u8) -> u8 {
    match val {
        b'a'..=b'z' => 1 + val - b'a',
        b'A'..=b'Z' => 27 + val - b'A',
        _ => 0,
    }
}

impl Rucksack {
    fn from_string(s: String) -> Rucksack {
        /*let mut chunks = s.as_bytes().chunks(s.len() / 2);
        Rucksack {
            left: chunks.next().unwrap().to_vec(),
            right: chunks.next().unwrap().to_vec(),
        }*/
        Rucksack {
            items: s.into_bytes(),
        }
    }

    fn misplaced_item(&self) -> u8 {
        let mut chunks = self.items.chunks(self.items.len() / 2);
        let left_set: HashSet<u8> = chunks.next().unwrap().iter().map(|x| *x).collect();
        let right_set: HashSet<u8> = chunks.next().unwrap().iter().map(|x| *x).collect();
        *left_set.intersection(&right_set).next().unwrap()
    }
}

fn read_input() -> Vec<Rucksack> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(Rucksack::from_string)
        .collect()
}

fn badge(group: &[Rucksack]) -> u8 {
    let mut i = group.iter();
    let mut set: HashSet<u8> = i.next().unwrap().items.iter().map(|s| *s).collect();
    for sack in i {
        let other: HashSet<u8> = sack.items.iter().map(|s| *s).collect();
        let intersect = set.intersection(&other);
        set = intersect.map(|s| *s).collect();
    }
    *set.iter().next().unwrap()
}

fn main() {
    let sacks = read_input();
    let part1: u32 = sacks
        .iter()
        .map(|s| priority(s.misplaced_item()) as u32)
        .sum();
    println!("part1 priority: {}", part1);

    let part2: u32 = sacks.chunks(3).map(|g| priority(badge(g)) as u32).sum();
    println!("part2 priority: {}", part2);
}
