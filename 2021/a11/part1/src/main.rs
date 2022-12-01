use std::io::{self, BufRead};

fn read_line(buf: &String) -> Vec<u8> {
    buf
        .chars()
        .map(|c| c.to_digit(10) )
        .filter(Option::is_some)
        .map(|c| c.unwrap() as u8)
        .collect::<Vec<_>>()
}

fn read_input() -> Vec<Vec<u8>> {
    io::stdin().lock().lines()
        .map(|line| read_line(&line.ok().unwrap()) )
        .collect()
}

fn print_board(board: &Vec<Vec<u8>>) {
    let rows = board.len();
    let cols = board[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let value = board[row][col];
            if value < 10 {
                print!("{}", value);
            } else {
                print!("0");
            }
        }
        print!("\n")
    }
}

fn flash(board: &mut Vec<Vec<u8>>, row: usize, col: usize) -> u32 {
    let rows = board.len();
    let cols = board[0].len();
    let mut flashes = 1;
    let adjacents = vec![
        (row as i32 - 1, col as i32 - 1),
        (row as i32 - 1, col as i32),
        (row as i32 - 1, col as i32 + 1),
        (row as i32 + 1, col as i32 - 1),
        (row as i32 + 1, col as i32),
        (row as i32 + 1, col as i32 + 1),
        (row as i32, col as i32 - 1),
        (row as i32, col as i32 + 1),
    ].iter()
        .filter(|(r,c)| *r >= 0 && *c >= 0 && *r < rows as i32 && *c < cols as i32)
        .map(|(r,c)| (*r as usize, *c as usize)).collect::<Vec<_>>();
    for (r,c) in adjacents {
        board[r][c] += 1;
        if board[r][c] == 10 {
            flashes += flash(board, r, c);
        }
    }

    flashes
}

fn step(board: &mut Vec<Vec<u8>>) -> u32 {
    let rows = board.len();
    let cols = board[0].len();
    let mut flashes = 0;

    // increase and flash
    for row in 0..rows {
        for col in 0..cols {
            board[row][col] += 1;
            if board[row][col] == 10 {
                flashes += flash(board, row, col);
            }
        }
    }

    // reset flashed to 0
    for row in 0..rows {
        for col in 0..cols {
            if board[row][col] > 9 {
                board[row][col] = 0;
            }
        }
    }
    flashes
}

fn main() {
    let mut board = read_input();
    let rows = board.len();
    let cols = board[0].len();
    
    //println!("Start:");
    //print_board(&board);
    
    let mut flashes = 0;
    for n in 1..=i32::MAX {
        let step_flashes = step(&mut board);
        if n <= 100 {
            flashes += step_flashes;
        }
        if step_flashes == (rows * cols) as u32 {
            println!("Sync flash on step {}", n);
            if n > 100 {
                break;
            }
        }
        //println!("Step {} - {} flashes:", n, step_flashes);
        //print_board(&board);
    }

    println!("Flashes after 100: {}", flashes);
}
