mod types;
mod input;

use crate::types::*;

fn main() {
    let start = Position{horizontal: 0, depth: 0};
    let end = input::INPUT.iter().fold(start, |pos, cmd| pos + *cmd);
    println!("the end is {}", end.horizontal * end.depth);
}
