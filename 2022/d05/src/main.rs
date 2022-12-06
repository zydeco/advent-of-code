use std::{
    fmt::{Debug, Display},
    io::{self, Read},
    u8,
};

struct Move {
    count: u8,
    from: u8,
    to: u8,
}

#[derive(Clone)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl Cargo {
    fn from_str(s: &str) -> Cargo {
        let lines = s.lines().collect::<Vec<_>>();
        let num_stacks = lines
            .last()
            .unwrap()
            .split(char::is_whitespace)
            .filter(|s| !s.is_empty())
            .count();
        println!("{} stacks!", num_stacks);
        if num_stacks > 9 {
            panic!("No more than 9 stacks supported");
        }
        let mut cargo = Cargo {
            stacks: vec![vec![]; num_stacks],
        };

        for pos in (0..lines.len() - 1).rev() {
            let line = lines[pos].chars().collect::<Vec<_>>();
            for stack in 0..num_stacks {
                let item = line[1 + 4 * stack];
                if !item.is_whitespace() {
                    cargo.stacks[stack].push(item);
                }
            }
        }

        cargo
    }

    fn apply(&self, mv: &Move, one_by_one: bool) -> Cargo {
        let from_idx = mv.from as usize;
        let to_idx = mv.to as usize;
        let moves = mv.count as usize;
        let mut from_stack = self.stacks.get(from_idx).unwrap().clone();
        let mut to_stack = self.stacks.get(to_idx).unwrap().clone();

        if one_by_one {
            for _ in 0..moves {
                to_stack.push(from_stack.pop().unwrap());
            }
        } else {
            let drain_range = from_stack.len() - moves..from_stack.len();
            for i in from_stack.drain(drain_range) {
                to_stack.push(i);
            }
        }

        Cargo {
            stacks: self
                .stacks
                .iter()
                .enumerate()
                .map(|(idx, stack)| match idx {
                    _ if idx == from_idx => from_stack.clone(),
                    _ if idx == to_idx => to_stack.clone(),
                    _ => stack.clone(),
                })
                .collect(),
        }
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|s| s.last())
            .map(|c| *c)
            .collect()
    }
}

impl Move {
    fn from_str(s: &str) -> Move {
        let words = s.split(' ').collect::<Vec<_>>();
        assert!(words[0].eq("move"));
        assert!(words[2].eq("from"));
        assert!(words[4].eq("to"));
        Move {
            count: words[1].parse().unwrap(),
            from: words[3].parse::<u8>().unwrap() - 1u8,
            to: words[5].parse::<u8>().unwrap() - 1u8,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.count,
            1 + self.from,
            1 + self.to
        )
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self::Display::fmt(self, f)
    }
}

impl Display for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, stack) in self.stacks.iter().enumerate() {
            write!(f, "{}: {:?}\n", 1 + idx, stack)?
        }
        Ok(())
    }
}

fn read_input() -> (Cargo, Vec<Move>) {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let mut i = buf.split("\n\n");
    let cargo = Cargo::from_str(i.next().unwrap());
    let moves = i
        .next()
        .unwrap()
        .split('\n')
        .filter(|ln| !ln.is_empty())
        .map(|ln| Move::from_str(ln))
        .collect::<Vec<_>>();
    (cargo, moves)
}

fn main() {
    let (mut cargo, moves) = read_input();
    let initial = cargo.clone();
    println!("Starting point:\n{}", initial);

    for mv in moves.iter() {
        cargo = cargo.apply(mv, true);
    }

    println!("Part1 position:\n{}Tops: {}", cargo, cargo.tops());

    cargo = initial.clone();
    for mv in moves.iter() {
        cargo = cargo.apply(mv, false);
    }
    println!("Part2 position:\n{}Tops: {}", cargo, cargo.tops());
}
