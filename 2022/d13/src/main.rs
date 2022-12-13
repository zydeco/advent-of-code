use std::{
    cmp::Ordering,
    fmt::Display,
    io::{self, BufRead},
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Packet::read_list(&mut s.chars().peekable()))
    }
}

impl Packet {
    fn read_integer(chars: &mut Peekable<Chars>) -> Self {
        let mut value = 0u8;
        while chars.peek().unwrap().is_digit(10) {
            value = value * 10u8 + chars.next().unwrap().to_digit(10).unwrap() as u8;
        }
        Packet::Integer(value)
    }

    fn read_list(chars: &mut Peekable<Chars>) -> Self {
        assert!(chars.next().unwrap() == '[');
        let mut items = vec![];
        if *chars.peek().unwrap() == ']' {
            return Packet::List(items);
        }
        loop {
            // read one element
            match chars.peek().unwrap() {
                '[' => items.push(Self::read_list(chars)),
                '0'..='9' => items.push(Self::read_integer(chars)),
                x => panic!("Expected opening bracket or digit, got '{}'", x),
            }
            // continue
            match chars.next().unwrap() {
                ',' => continue,
                ']' => return Packet::List(items),
                x => panic!("Expected comma or closing bracket, got '{}'", x),
            }
        }
    }

    fn divider(value: u8) -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Integer(value)])])
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(x) => {
                write!(f, "{}", x)?;
            }
            Packet::List(items) => {
                write!(f, "[")?;
                let last = items.len().saturating_sub(1);
                for (idx, packet) in items.iter().enumerate() {
                    write!(f, "{}", packet)?;
                    if idx < last {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

fn read_input() -> Vec<Packet> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| Packet::from_str(&line).unwrap())
        .collect()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => x.cmp(y),
            (Packet::List(_), Packet::Integer(y)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*y)]))
            }
            (Packet::Integer(x), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*x)]).cmp(other)
            }
            (Packet::List(x), Packet::List(y)) => x.cmp(y),
        }
    }
}

fn main() {
    let mut input = read_input();

    let ordered_indices: usize = input
        .chunks(2)
        .enumerate()
        .map(|(idx, chunk)| (idx, chunk[0].cmp(&chunk[1])))
        .filter(|(idx, order)| order.is_lt())
        .map(|(idx, order)| 1 + idx)
        .sum();

    println!("part1: {}", ordered_indices);

    // part 2
    let div2 = Packet::divider(2);
    let div6 = Packet::divider(6);
    input.push(Packet::divider(2));
    input.push(Packet::divider(6));
    input.sort();
    let pos2 = 1 + input.iter().position(|p| p.eq(&div2)).unwrap();
    let pos6 = pos2 + 1 + input.iter().skip(pos2).position(|p| p.eq(&div6)).unwrap();
    println!("Part 2: {}", pos2 * pos6);
}
