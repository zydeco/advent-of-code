mod types;

use crate::types::*;
use std::io::{self, BufRead};

fn parse(line: String) -> Command {
    let mut words = line.split_whitespace();
    let direction = match words.next().unwrap() {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        wtf => panic!("unknown direction {}", wtf)
    };
    let value = words.next().unwrap();
    Command { direction: direction, value: value.parse().unwrap() }
}

fn main() {
    let stdin = io::stdin();
    let mut ops: Vec<Command> = Vec::new();

    for line in stdin.lock().lines() {
        ops.push(parse(line.unwrap()));
    }

    let end = ops.iter().fold(Position{horizontal: 0, depth: 0, aim: 0}, |pos, cmd| pos + cmd);
    println!("the end is {}", end.horizontal * end.depth);
}

