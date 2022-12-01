mod board;

use std::io::{self, BufRead};
use crate::board::*;

fn read_input() -> Result<(Vec<u8>, Vec<Board>), std::io::Error> {
    let stdin = io::stdin();
    let mut boards: Vec<Board> = Vec::new();
    
    let mut buf = String::new();
    let mut reader = stdin.lock();
    // read numbers
    reader.read_line(&mut buf)?;
    let numbers = parse_numbers(&buf);

    // read boards
    while reader.read_line(&mut buf).ok() != Some(0) {
        buf.clear();
        for _ in 0..5 { reader.read_line(&mut buf)?; }
        let board_numbers = parse_numbers(&buf);
        boards.push(Board::new(board_numbers.as_slice()));
    }

    return Ok((numbers, boards));
}

fn parse_numbers(buf: &String) -> Vec<u8> {
    return buf.split(|c: char| !c.is_numeric() )
        .filter(|s| !s.is_empty() )
        .map(|s| u8::from_str_radix(s, 10).unwrap())
        .collect();
}

fn main() {
    let (numbers, mut boards) = read_input().unwrap();

    for n in numbers {
        for board in boards.iter_mut() {
            let was_winning = board.is_winning();
            board.call(n);
            if board.is_winning() && !was_winning {
                let score = board.score();
                // first winner = part 1, last winner = part 2
                println!("winner! {}*{} = {}", score, n, score*(n as u32));
            }
        }
    }
}
