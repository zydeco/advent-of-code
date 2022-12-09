use std::{
    collections::HashSet,
    io::{self, BufRead},
    ops::Range,
    str::FromStr,
};

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

macro_rules! pos {
    ($a:expr,$b:expr) => {{
        Pos { x: $a, y: $b }
    }};
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    len: i32,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Pos>,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Invalid length: '{}'", s));
        }
        match s.chars().next().unwrap() {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            x => Err(format!("Invalid direction: '{}'", x)),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(char::is_whitespace).collect::<Vec<_>>();
        if words.len() != 2 {
            return Err(format!("Too many words: '{}'", s));
        }
        Ok(Move {
            dir: words[0].parse().unwrap(),
            len: words[1].parse().unwrap(),
        })
    }
}

impl Pos {
    fn is_next_to(&self, other: &Pos) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn follow(&self, other: &Pos) -> Pos {
        assert!(!self.is_next_to(other));
        let dx = if other.x > self.x {
            1
        } else if other.x < self.x {
            -1
        } else {
            0
        };
        let dy = if other.y > self.y {
            1
        } else if other.y < self.y {
            -1
        } else {
            0
        };
        pos!(self.x + dx, self.y + dy)
    }

    fn step(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::Up => pos!(self.x, self.y - 1),
            Direction::Down => pos!(self.x, self.y + 1),
            Direction::Left => pos!(self.x - 1, self.y),
            Direction::Right => pos!(self.x + 1, self.y),
        }
    }
}

impl Rope {
    fn new(size: usize) -> Rope {
        assert!(size > 1);
        Rope {
            knots: vec![Pos::default(); size],
        }
    }

    fn tail(&self) -> Pos {
        *self.knots.last().unwrap()
    }

    fn apply(&self, mv: &Move, tail_visitor: &mut HashSet<Pos>) -> Rope {
        let dir = mv.dir;
        let init = self.step(&dir, tail_visitor);
        (1..mv.len).fold(init, |acc, _| acc.step(&dir, tail_visitor))
    }

    fn step(&self, direction: &Direction, tail_visitor: &mut HashSet<Pos>) -> Rope {
        let head = self.knots[0];
        let mut knots = self.knots.clone();
        knots[0] = head.step(direction);
        for i in 1..knots.len() {
            let prev = knots[i - 1];
            if !prev.is_next_to(&knots[i]) {
                knots[i] = knots[i].follow(&prev);
            }
        }

        // tail
        tail_visitor.insert(self.tail());

        Rope { knots }
    }
}

fn read_input() -> Vec<Move> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| Move::from_str(&line))
        .filter_map(Result::ok)
        .collect()
}

fn print_rope(rope: &Rope, corners: Range<Pos>, visited: &HashSet<Pos>) {
    for y in corners.start.y..corners.end.y {
        for x in corners.start.x..corners.end.x {
            let pos = Pos { x: x, y: y };
            print!(
                "{}",
                match rope.knots.iter().position(|&p| p == pos) {
                    Some(0) => 'H',
                    Some(1) if rope.knots.len() == 2 => 'T',
                    Some(x) => char::from_digit(x as u32, 10).unwrap_or('?'),
                    None =>
                        if pos == Pos::default() {
                            's'
                        } else if visited.contains(&pos) {
                            '#'
                        } else {
                            '.'
                        },
                }
            );
        }
        println!("");
    }
}

fn tail_visits(moves: &Vec<Move>, rope_size: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(Pos::default());
    moves
        .iter()
        .fold(Rope::new(rope_size), |r, mv| r.apply(mv, &mut visited));
    visited.len()
}

fn main() {
    let moves = read_input();

    println!("Part1: visited {} points", tail_visits(&moves, 2));
    println!("Part2: visited {} points", tail_visits(&moves, 10));
}
