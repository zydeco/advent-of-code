use std::io::{self, BufRead};

fn read_input() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .next()
        .unwrap()
        .split(',')
        .map(str::to_string)
        .collect()
}

fn hash(s: &String) -> u8 {
    s.as_bytes().iter().fold(0, |acc, next| {
        ((((acc as u16) + (*next as u16)) * 17) % 256) as u8
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Add(String, u8),
    Remove(String),
}

impl Instruction {
    fn from_string(s: &String) -> Instruction {
        let len = s.len();
        if s.ends_with('-') {
            Self::Remove(s[0..len - 1].to_string())
        } else if Some('=').eq(&s.chars().rev().skip(1).next()) {
            let value = u8::from_str_radix(&s[len - 1..len], 10).unwrap();
            Self::Add(s[0..len - 2].to_string(), value)
        } else {
            panic!("Unparsable instruction: {}", s);
        }
    }

    fn hash(&self) -> u8 {
        hash(match self {
            Instruction::Add(label, _) => label,
            Instruction::Remove(label) => label,
        })
    }
}

fn part2(input: Vec<Instruction>) {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    for i in input {
        let target = &mut boxes[i.hash() as usize];
        //println!("After {:?}:", i);
        match i {
            Instruction::Add(label, value) => {
                if let Some(pos) = target.iter().position(|(l, _)| label.eq(l)) {
                    target[pos].1 = value
                } else {
                    target.push((label, value))
                }
            }
            Instruction::Remove(label) => {
                if let Some(pos) = target.iter().position(|(l, _)| label.eq(l)) {
                    target.remove(pos);
                }
            }
        }

        /*for (n, b) in boxes.iter().enumerate() {
            if !b.is_empty() {
                println!("Box {}: {:?}", n, b);
            }
        }
        println!("");*/
    }

    let value: u64 = boxes
        .iter()
        .enumerate()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(slot_number, &(_, focal_length))| {
                    (box_number as u64 + 1) * (slot_number as u64 + 1) * focal_length as u64
                })
                .sum::<u64>()
        })
        .sum();
    println!("Part 2: {}", value);
}

fn main() {
    let input = read_input();
    let part1 = input
        .iter()
        .map(hash)
        .fold(0u32, |acc, hash| acc + hash as u32);
    println!("Part 1: {}", part1);

    part2(input.iter().map(Instruction::from_string).collect());
}
