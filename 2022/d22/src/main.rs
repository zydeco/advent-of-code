use std::{
    fmt::Display,
    io::{self, Read},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

struct Board {
    lines: Vec<(usize, Vec<Tile>)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Player {
    row: usize,
    col: usize,
    facing: Direction,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => panic!("Unknown tile {}", c),
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            x if x.is_digit(10) => Self::Forward(x.to_digit(10).unwrap() as usize),
            _ => panic!("Unknown move {}", c),
        }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = vec![];
        for line in s.split("\n").filter(|ln| !ln.is_empty()) {
            let start = line.find(|c| c != ' ').unwrap();
            let tiles = line[start..].chars().map(char::into).collect();
            lines.push((start, tiles))
        }
        Ok(Board { lines })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Wall => '#',
            }
        )
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (start, tiles) in self.lines.iter() {
            for _ in 0..*start {
                write!(f, " ")?
            }
            for t in tiles {
                write!(f, "{}", t)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Board {
    fn starting_position(&self) -> Player {
        Player {
            row: 0,
            col: self.lines[0].0,
            facing: Direction::Right,
        }
    }

    fn tile(&self, row: usize, col: usize) -> Option<Tile> {
        if row >= self.lines.len() {
            return None;
        }
        let (start, tiles) = self.lines.get(row).unwrap();
        if col < *start || col - *start >= tiles.len() {
            return None;
        }
        return Some(tiles[col - *start]);
    }

    fn adjacent(&self, row: usize, col: usize, direction: Direction) -> (usize, usize) {
        match direction {
            Direction::Up => self.adjacent_up(row, col),
            Direction::Right => self.adjacent_right(row, col),
            Direction::Down => self.adjacent_down(row, col),
            Direction::Left => self.adjacent_left(row, col),
        }
    }

    fn adjacent_empty(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let adjacent = self.adjacent(row, col, direction);
        if self.tile(adjacent.0, adjacent.1).unwrap() == Tile::Empty {
            Some(adjacent)
        } else {
            None
        }
    }

    fn adjacent_up(&self, row: usize, col: usize) -> (usize, usize) {
        if row == 0 || self.tile(row - 1, col).is_none() {
            // wrap
            let mut new_row = self.lines.len() - 1;
            while self.tile(new_row, col).is_none() {
                new_row -= 1;
            }
            (new_row, col)
        } else {
            (row - 1, col)
        }
    }

    fn adjacent_right(&self, row: usize, col: usize) -> (usize, usize) {
        if self.tile(row, col + 1).is_none() {
            // wrap
            let mut new_col = 0;
            while self.tile(row, new_col).is_none() {
                new_col += 1;
            }
            (row, new_col)
        } else {
            (row, col + 1)
        }
    }

    fn adjacent_down(&self, row: usize, col: usize) -> (usize, usize) {
        if self.tile(row + 1, col).is_none() {
            // wrap
            let mut new_row = 0;
            while self.tile(new_row, col).is_none() {
                new_row += 1;
            }
            (new_row, col)
        } else {
            (row + 1, col)
        }
    }

    fn adjacent_left(&self, row: usize, col: usize) -> (usize, usize) {
        if col == 0 || self.tile(row, col - 1).is_none() {
            // wrap
            let (start, tiles) = self.lines.get(row).unwrap();
            let mut new_col = start + tiles.len();
            while self.tile(row, new_col).is_none() {
                new_col -= 1;
            }
            (row, new_col)
        } else {
            (row, col - 1)
        }
    }
}

impl Direction {
    fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn rotate(&self, mv: &Move) -> Direction {
        match mv {
            Move::Left => self.rotate_counter_clockwise(),
            Move::Right => self.rotate_clockwise(),
            _ => *self,
        }
    }
}

impl Player {
    fn go(&self, board: &Board, mv: &Move) -> Player {
        match mv {
            Move::Forward(x) => self.go_forward(board, *x),
            Move::Left | Move::Right => Player {
                row: self.row,
                col: self.col,
                facing: self.facing.rotate(mv),
            },
        }
    }

    fn go_forward(&self, board: &Board, mut length: usize) -> Player {
        let (mut row, mut col) = (self.row, self.col);
        while length > 0 {
            if let Some(next) = board.adjacent_empty(row, col, self.facing) {
                length -= 1;
                (row, col) = next;
            } else {
                break;
            }
        }
        Player {
            row,
            col,
            facing: self.facing,
        }
    }

    fn password(&self) -> usize {
        1000 * (1 + self.row) + 4 * (1 + self.col) + self.facing as usize
    }
}

fn read_input() -> (Board, Vec<Move>) {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let (board_str, moves_str) = buf.split_at(buf.find("\n\n").unwrap());
    (Board::from_str(board_str).unwrap(), parse_moves(moves_str))
}

fn parse_moves(s: &str) -> Vec<Move> {
    let mut d = None;
    let mut moves = vec![];
    for c in s.chars() {
        if let Some(value) = c.to_digit(10) {
            d = Some(d.unwrap_or(0) * 10 + value as usize);
        } else if c == 'L' || c == 'R' {
            if let Some(f) = d {
                moves.push(Move::Forward(f));
            }
            moves.push(c.into());
            d = None;
        }
    }
    if let Some(f) = d {
        moves.push(Move::Forward(f));
    }
    moves
}

fn main() {
    let (board, moves) = read_input();

    let mut player = board.starting_position();
    for mv in moves {
        player = player.go(&board, &mv);
    }
    println!("Part1: {}", player.password());
}
