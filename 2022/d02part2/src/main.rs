use std::io::{self, Read};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Game {
    opponent: Shape,
    outcome: Outcome,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Draw = 1,
    Win = 2,
}

impl Shape {
    fn from_char(c: char) -> Shape {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("Unknown shape {}", c),
        }
    }

    fn win(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn lose(self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

impl Outcome {
    fn from_char(c: char) -> Outcome {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unknown outcome {}", c),
        }
    }
}

impl Game {
    fn from_str(input: &str) -> Game {
        let mut chars = input.chars();
        let opponent = chars.next().unwrap();
        let outcome = chars.nth(1).unwrap();
        Game {
            opponent: Shape::from_char(opponent),
            outcome: Outcome::from_char(outcome),
        }
    }

    fn score(self) -> u32 {
        (3 * self.outcome as u32) + self.player() as u32
    }

    fn player(self) -> Shape {
        match (self.outcome, self.opponent) {
            (Outcome::Draw, x) => x,
            (Outcome::Win, x) => x.win(),
            (Outcome::Loss, x) => x.lose(),
        }
    }
}

fn read_input() -> Vec<Game> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    buf.split('\n')
        .filter(|line| !line.is_empty())
        .map(Game::from_str)
        .collect()
}

fn main() {
    let games = read_input();

    println!("{} games", games.len());

    let score: u32 = games.iter().map(|g| g.score()).sum();

    println!("Score {}", score);
}
