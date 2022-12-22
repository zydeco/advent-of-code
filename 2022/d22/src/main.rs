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

type WrappingEdge = (
    Direction, // exit direction
    usize,     // exit row/col min
    usize,     // exit row/col max (inclusive)
    Direction, // entry direction
    // entry range can be reversed when flipping
    usize, // entry row/col range min/max
    usize, // entry row/col range min/max
    // exit and entry col/row - could be calculated based on map
    usize, // exit col/row
    usize, // entry col/row
);
type WrappingMap = [WrappingEdge; 14];

struct Board {
    lines: Vec<(usize, Vec<Tile>)>,
    wrapping: WrappingMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Forward(usize),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
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
        let num_lines = lines.len();
        Ok(Board {
            lines,
            wrapping: wrapping_map(num_lines),
        })
    }
}

pub fn wrapping_map(lines: usize) -> WrappingMap {
    let map = if lines == 12 {
        // sample input
        [
            (Direction::Up, 8, 11, Direction::Down, 3, 0, 0, 4),
            (Direction::Left, 0, 3, Direction::Down, 4, 7, 8, 4),
            (Direction::Left, 4, 7, Direction::Up, 15, 12, 0, 11),
            (Direction::Down, 0, 3, Direction::Up, 11, 8, 7, 11),
            (Direction::Down, 4, 7, Direction::Right, 11, 8, 7, 8),
            (Direction::Right, 0, 3, Direction::Left, 11, 8, 11, 15),
            (Direction::Right, 4, 7, Direction::Down, 15, 12, 11, 8),
        ]
    } else {
        // my input
        [
            (Direction::Left, 0, 49, Direction::Right, 149, 100, 50, 0),
            (Direction::Left, 50, 99, Direction::Down, 0, 49, 50, 100),
            (Direction::Left, 150, 199, Direction::Down, 50, 99, 0, 0),
            (Direction::Right, 0, 49, Direction::Left, 149, 100, 149, 99),
            (Direction::Right, 50, 99, Direction::Up, 100, 149, 99, 49),
            (Direction::Right, 150, 199, Direction::Up, 50, 99, 49, 149),
            (Direction::Down, 0, 49, Direction::Down, 100, 149, 199, 0),
        ]
    };

    // calculate mirrored wrapping
    [
        map[0],
        wrapping_edge_mirror(&map[0]),
        map[1],
        wrapping_edge_mirror(&map[1]),
        map[2],
        wrapping_edge_mirror(&map[2]),
        map[3],
        wrapping_edge_mirror(&map[3]),
        map[4],
        wrapping_edge_mirror(&map[4]),
        map[5],
        wrapping_edge_mirror(&map[5]),
        map[6],
        wrapping_edge_mirror(&map[6]),
    ]
}

pub fn wrapping_edge_mirror(edge: &WrappingEdge) -> WrappingEdge {
    if edge.4 < edge.5 {
        (
            edge.3.turn_back(),
            edge.4,
            edge.5,
            edge.0.turn_back(),
            edge.1,
            edge.2,
            edge.7,
            edge.6,
        )
    } else {
        (
            edge.3.turn_back(),
            edge.5,
            edge.4,
            edge.0.turn_back(),
            edge.2,
            edge.1,
            edge.7,
            edge.6,
        )
    }
}

pub fn wrap(
    map: &WrappingMap,
    row: usize,
    col: usize,
    facing: Direction,
) -> (usize, usize, Direction) {
    let (edge_coord, exit_coord) = if facing.is_horizontal() {
        (row, col)
    } else {
        (col, row)
    };
    let edge = map
        .iter()
        .find(|e| e.0 == facing && edge_coord >= e.1 && edge_coord <= e.2)
        .unwrap();
    let new_edge_coord = if edge.4 < edge.5 {
        edge.4 + (edge_coord - edge.1)
    } else {
        edge.4 - (edge_coord - edge.1)
    };
    assert_eq!(exit_coord, edge.6);
    let entry_coord = edge.7;
    match (facing.is_horizontal(), edge.3.is_horizontal()) {
        (true, true) => (new_edge_coord, entry_coord, edge.3),
        (true, false) => (entry_coord, new_edge_coord, edge.3),
        (false, false) => (entry_coord, new_edge_coord, edge.3),
        (false, true) => (new_edge_coord, entry_coord, edge.3),
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

    fn next(&self, row: usize, col: usize, direction: Direction) -> (usize, usize, Direction) {
        match direction {
            Direction::Up => self.next_up(row, col),
            Direction::Right => self.next_right(row, col),
            Direction::Down => self.next_down(row, col),
            Direction::Left => self.next_left(row, col),
        }
    }

    fn next_unoccupied(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<(usize, usize, Direction)> {
        let adjacent = self.next(row, col, direction);
        if self.tile(adjacent.0, adjacent.1).unwrap() == Tile::Empty {
            Some(adjacent)
        } else {
            None
        }
    }

    fn next_up(&self, row: usize, col: usize) -> (usize, usize, Direction) {
        if row == 0 || self.tile(row - 1, col).is_none() {
            wrap(&self.wrapping, row, col, Direction::Up)
        } else {
            (row - 1, col, Direction::Up)
        }
    }

    fn next_right(&self, row: usize, col: usize) -> (usize, usize, Direction) {
        if self.tile(row, col + 1).is_none() {
            wrap(&self.wrapping, row, col, Direction::Right)
        } else {
            (row, col + 1, Direction::Right)
        }
    }

    fn next_down(&self, row: usize, col: usize) -> (usize, usize, Direction) {
        if self.tile(row + 1, col).is_none() {
            wrap(&self.wrapping, row, col, Direction::Down)
        } else {
            (row + 1, col, Direction::Down)
        }
    }

    fn next_left(&self, row: usize, col: usize) -> (usize, usize, Direction) {
        if col == 0 || self.tile(row, col - 1).is_none() {
            wrap(&self.wrapping, row, col, Direction::Left)
        } else {
            (row, col - 1, Direction::Left)
        }
    }
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn turn_back(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn rotate(&self, mv: &Move) -> Direction {
        match mv {
            Move::Left => self.rotate_counter_clockwise(),
            Move::Right => self.rotate_clockwise(),
            _ => *self,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.eq(&Direction::Left) || self.eq(&Direction::Right)
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
        let (mut row, mut col, mut facing) = (self.row, self.col, self.facing);
        while length > 0 {
            if let Some(next) = board.next_unoccupied(row, col, facing) {
                length -= 1;
                (row, col, facing) = next;
            } else {
                break;
            }
        }
        Player { row, col, facing }
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
    println!("Password: {}", player.password());
}

#[cfg(test)]
mod tests {
    use crate::{wrap, wrapping_edge_mirror, wrapping_map, Direction, WrappingMap};

    #[test]
    fn test_small_cube_wrapping() {
        let w = wrapping_map(12);
        check_wrapping_map(&w);
        check_wrap(&w, (5, 11, Direction::Right), (8, 14, Direction::Down));
        check_wrap(&w, (11, 10, Direction::Down), (7, 1, Direction::Up));
        check_wrap(&w, (2, 11, Direction::Right), (9, 15, Direction::Left));
        check_wrap(&w, (11, 13, Direction::Down), (6, 0, Direction::Right));
        check_wrap(&w, (7, 5, Direction::Down), (10, 8, Direction::Right));
        check_wrap(&w, (4, 1, Direction::Up), (0, 10, Direction::Down));
        check_wrap(&w, (4, 5, Direction::Up), (1, 8, Direction::Right));
    }

    fn check_wrap(
        w: &WrappingMap,
        exit: (usize, usize, Direction),
        entry: (usize, usize, Direction),
    ) {
        assert_eq!(wrap(&w, exit.0, exit.1, exit.2), entry);
        assert_eq!(
            wrap(&w, entry.0, entry.1, entry.2.turn_back()),
            (exit.0, exit.1, exit.2.turn_back())
        );
    }

    fn check_wrapping_map(map: &WrappingMap) {
        let diff = (map[0].1 as i64 - map[0].2 as i64).abs();
        for i in 0..14 {
            let m = map[i];
            assert_eq!(diff, (m.1 as i64 - m.2 as i64).abs(), "from {}", i);
            assert_eq!(diff, (m.4 as i64 - m.5 as i64).abs(), "to {}", i);
            // find corresponding opposite
            let w = wrapping_edge_mirror(&m);
            assert_ne!(m, w);
            assert!(map.contains(&w), "map should contain {:?}", w);
        }
    }
}
