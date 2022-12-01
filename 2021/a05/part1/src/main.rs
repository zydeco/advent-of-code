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
        assert!(self.is_orthogonal());
        for x in self.x_range() {
            for y in self.y_range() {
                op(x, y)
            }
        }
    }
}

fn read_input() -> Vec<Line> {
    io::stdin().lock().lines()
        .map(|line| Line::from_string(&line.unwrap()) )
        .filter(Line::is_orthogonal)
        .collect()
}

fn bounds(lines: &Vec<Line>) -> (usize, usize) {
    (
        lines.into_iter().map(|line| line.from.y.max(line.to.y)).max().unwrap_or(0) as usize,
        lines.into_iter().map(|line| line.from.x.max(line.to.x)).max().unwrap_or(0) as usize
    )
}

fn print_board(board: &Vec<Vec<u8>>) {
    for line in board {
        for b in line {
            print!("{}", match b {
                0 => '.',
                1 => '*',
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

    println!("bounds {}x{}", bounds.0, bounds.1);

    for line in lines {
        line.plot(|x, y| board[x as usize][y as usize] += 1);
    }

    print_board(&board);
    let result = board.into_iter().flatten().filter(|x| *x > 1).fold(0, |sum, _| sum + 1);
    println!("result = {}", result);
}
