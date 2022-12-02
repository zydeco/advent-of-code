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
    player: Shape,
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
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Unknown shape {}", c),
        }
    }
}

impl Game {
    fn from_str(input: &str) -> Game {
        let mut chars = input.chars();
        let opponent = chars.next().unwrap();
        let player = chars.nth(1).unwrap();
        Game {
            opponent: Shape::from_char(opponent),
            player: Shape::from_char(player),
        }
    }

    fn score(self) -> u32 {
        (3 * self.outcome() as u32) + self.player as u32
    }

    fn outcome(self) -> Outcome {
        match (self.player, self.opponent) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Paper, Shape::Rock) => Outcome::Win,
            (x, y) if (x == y) => Outcome::Draw,
            _ => Outcome::Loss,
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
