use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: u16,
    y: u16
}

#[derive(Copy, Clone, Debug)]
struct Line {
    from: Point,
    to: Point
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{},{} -> {},{}", self.from.x, self.from.y, self.to.x, self.to.y)
    }
}

impl Line {
    fn new(values: &[u16]) -> Line {
        Line{
            from: Point{x: values[0], y: values[1]},
            to: Point{x: values[2], y: values[3]}
        }
    }

    fn from_string(string: &String) -> Line {
        Line::new(string
            .split(|c: char| !c.is_numeric() )
            .filter(|s| !s.is_empty() )
            .map(|s| u16::from_str_radix(s, 10).unwrap())
            .collect::<Vec<_>>()
            .as_slice())
    }

    fn is_orthogonal(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    fn is_diagonal(&self) -> bool {
        let x_range = self.x_range();
        let y_range = self.y_range();
        let x_size = x_range.end() - x_range.start();
        let y_size = y_range.end() - y_range.start();
        x_size == y_size
    }

    fn x_range(&self) -> std::ops::RangeInclusive<u16> {
        if self.from.x < self.to.x {
            self.from.x..=self.to.x
        } else {
            self.to.x..=self.from.x
        }
    }

    fn y_range(&self) -> std::ops::RangeInclusive<u16> {
        if self.from.y < self.to.y {
            self.from.y..=self.to.y
        } else {
            self.to.y..=self.from.y
        }
    }

    fn plot<F>(&self, mut op: F) where F: FnMut(u16, u16) -> () {
        assert!(self.is_orthogonal() || self.is_diagonal());
        if self.is_orthogonal() {
            for x in self.x_range() {
                for y in self.y_range() {
                    op(x, y)
                }
            }
        } else if self.is_diagonal() {
            let px: i16 = if self.to.x > self.from.x { 1 } else { -1 };
            let py: i16 = if self.to.y > self.from.y { 1 } else { -1 };
            let length = ((self.to.x as i16) - (self.from.x as i16)).abs();
            for i in 0..=length {
                op((self.from.x as i16 + px*i) as u16, (self.from.y as i16 + py*i) as u16)
            }
        } else {
            panic!("line is not orthogonal or diagonal, cannot plot")
        }
    }
}

fn read_input() -> Vec<Line> {
    io::stdin().lock().lines()
        .map(|line| Line::from_string(&line.unwrap()) )
        .filter(|ln| ln.is_orthogonal() || ln.is_diagonal() )
        .collect()
}

fn bounds(lines: &Vec<Line>) -> (usize, usize) {
    (
        lines.into_iter().map(|line| line.from.y.max(line.to.y)).max().unwrap_or(0) as usize,
        lines.into_iter().map(|line| line.from.x.max(line.to.x)).max().unwrap_or(0) as usize
    )
}

fn print_board(board: &Vec<Vec<u8>>) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            print!("{}", match board[x][y] {
                0 => '.',
                1 => '1',
                2..=9 => (0x30 + board[x][y]) as char,
                _ => 'X',
            })
        }
        print!("\n")
    }
}

fn main() {
    let lines = read_input();
    let bounds = bounds(&lines);
    let mut board = vec![vec![0u8; 1 + bounds.0]; 1 + bounds.1];

    for line in lines {
        line.plot(|x, y| board[x as usize][y as usize] += 1);
    }

    if bounds.0 < 80 {
        print_board(&board);
    }
    
    let result = board.into_iter().flatten().filter(|x| *x > 1).fold(0, |sum, _| sum + 1);
    println!("result = {}", result);
}
