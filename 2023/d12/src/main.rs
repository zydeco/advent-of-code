use std::{
    fmt::Display,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

type U = u64;

#[derive(Debug)]
struct Record {
    springs: Vec<Status>,
    damaged_groups: Vec<u8>,
}

// opposite of INTERCAL's select operator
fn spurn(input: U, arrangement: U) -> U {
    if input == 0 || arrangement == 0 || ((input & ((1 << arrangement.count_ones()) - 1)) == 0) {
        return 0;
    }
    let mut a = arrangement.next_power_of_two();
    if a != arrangement {
        a >>= 1;
    }
    let mut i = 1 << (arrangement.count_ones() - 1);
    let mut result = 0;

    while a != 0 {
        if arrangement & a == a {
            if input & i == i {
                result |= a;
            }
            i >>= 1;
        }
        a >>= 1;
    }
    result
}

fn groups_of_ones(input: U) -> Vec<u8> {
    let mut groups = vec![];
    let mut last_group = 0;
    let mut i = input.reverse_bits();
    while i != 0 {
        if i & 1 == 1 {
            last_group += 1;
        } else if last_group != 0 {
            groups.push(last_group);
            last_group = 0;
        }
        i >>= 1;
    }
    if last_group != 0 {
        groups.push(last_group);
    }
    groups
}

impl Status {
    fn bitmap(&self, input: &Vec<Status>) -> U {
        input
            .iter()
            .rev()
            .enumerate()
            .filter(|&(_, status)| self.eq(status))
            .map(|(idx, _)| 1 << idx)
            .fold(0, |acc, n| acc | n)
    }

    fn from_char(c: char) -> Self {
        match c {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!("Invalid status “{}”", c),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Operational => '.',
                Status::Damaged => '#',
                Status::Unknown => '?',
            }
        )
    }
}

impl Record {
    fn from_string(s: String) -> Self {
        let mut parts = s.split(' ');
        let springs: Vec<Status> = parts
            .next()
            .unwrap()
            .chars()
            .map(Status::from_char)
            .collect();
        let damaged_groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|word| u8::from_str_radix(word, 10).unwrap())
            .collect();
        Self {
            springs,
            damaged_groups,
        }
    }

    fn count_valid_arrangements(&self) -> usize {
        if self.springs.len() > 63 {
            return 0;
        }
        let unknowns = Status::Unknown.bitmap(&self.springs);
        let base = Status::Damaged.bitmap(&self.springs);
        let max = 1 << unknowns.count_ones();
        let mut count = 0;
        for i in 0..max {
            let variant = spurn(i, unknowns) | base;
            if groups_of_ones(variant).eq(&self.damaged_groups) {
                count += 1;
            }
        }

        count
    }
}

fn read_input() -> Vec<Record> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(Record::from_string)
        .collect()
}

fn sanity_checks() {
    assert_eq!(spurn(0b111011, 0b10101010101), 0b10101000101);
    assert_eq!(groups_of_ones(0b110110111), vec![2, 2, 3]);
    assert_eq!(groups_of_ones(0b11011011101), vec![2, 2, 3, 1]);
    assert_eq!(groups_of_ones(0), vec![]);
    assert_eq!(groups_of_ones(1), vec![1]);
    assert_eq!(groups_of_ones(0b111111), vec![6]);
    assert_eq!(
        Status::Damaged.bitmap(&vec![
            Status::Unknown,
            Status::Operational,
            Status::Damaged,
            Status::Damaged,
            Status::Operational,
            Status::Damaged,
            Status::Unknown,
            Status::Operational,
            Status::Damaged
        ]),
        0b1101001
    );
    assert_eq!(
        Status::Operational.bitmap(&vec![
            Status::Unknown,
            Status::Operational,
            Status::Damaged,
            Status::Damaged,
            Status::Operational,
            Status::Damaged,
            Status::Unknown,
            Status::Operational,
            Status::Damaged
        ]),
        0b010010010
    );
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in self.springs.iter() {
            write!(f, "{}", spring)?;
        }
        write!(
            f,
            " {}",
            self.damaged_groups
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )?;

        Ok(())
    }
}

fn main() {
    sanity_checks();
    let input = read_input();
    let sum_arrangements: usize = input.iter().map(Record::count_valid_arrangements).sum();
    println!("Part 1: {}", sum_arrangements);
}
