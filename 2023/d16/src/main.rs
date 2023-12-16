use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

type Pos = (i32, i32); // row, col

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    MirrorFront, // '/'
    MirrorBack,  // '\'
    SplitH,
    SplitV,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Beam = (Pos, Direction);

impl Direction {
    fn from(&self, pos: &Pos) -> Pos {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }

    fn beam(&self, pos: &Pos) -> Beam {
        (self.from(pos), *self)
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::MirrorFront,
            '\\' => Self::MirrorBack,
            '-' => Self::SplitH,
            '|' => Self::SplitV,
            _ => panic!("Invalid tile “{}”", c),
        }
    }

    fn beam(&self, beam: Beam) -> Vec<Beam> {
        let (pos, dir) = (&beam.0, beam.1);
        match (self, dir) {
            (Tile::Empty, _)
            | (Tile::SplitH, Direction::Left | Direction::Right)
            | (Tile::SplitV, Direction::Up | Direction::Down) => {
                vec![(dir.from(pos), dir)]
            }
            (Tile::SplitH, Direction::Up | Direction::Down) => {
                vec![Direction::Left.beam(pos), Direction::Right.beam(pos)]
            }
            (Tile::SplitV, Direction::Left | Direction::Right) => {
                vec![Direction::Up.beam(pos), Direction::Down.beam(pos)]
            }
            (Tile::MirrorFront, Direction::Up) => vec![Direction::Right.beam(pos)],
            (Tile::MirrorFront, Direction::Down) => vec![Direction::Left.beam(pos)],
            (Tile::MirrorFront, Direction::Left) => vec![Direction::Down.beam(pos)],
            (Tile::MirrorFront, Direction::Right) => vec![Direction::Up.beam(pos)],
            (Tile::MirrorBack, Direction::Up) => vec![Direction::Left.beam(pos)],
            (Tile::MirrorBack, Direction::Down) => vec![Direction::Right.beam(pos)],
            (Tile::MirrorBack, Direction::Left) => vec![Direction::Up.beam(pos)],
            (Tile::MirrorBack, Direction::Right) => vec![Direction::Down.beam(pos)],
        }
    }
}

#[derive(Debug)]
struct Board {
    cols: usize,
    rows: usize,
    tiles: HashMap<Pos, Tile>,
}

impl Board {
    fn get(&self, pos: &Pos) -> Option<Tile> {
        if self.is_valid_pos(pos) {
            self.tiles.get(pos).map(|t| *t).or(Some(Tile::Empty))
        } else {
            None
        }
    }

    fn is_valid_pos(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.rows as i32 && pos.1 < self.cols as i32
    }

    fn beam(&self, beam: Beam) -> Vec<Beam> {
        self.get(&beam.0)
            .map(|tile| {
                tile.beam(beam)
                    .into_iter()
                    .filter(|(pos, _)| self.is_valid_pos(pos))
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn energized(&self, start: &Beam) -> usize {
        let mut beams = vec![*start];
        let mut seen = HashSet::new();
        while let Some(beam) = beams.pop() {
            if !seen.contains(&beam) {
                seen.insert(beam);
                let mut next = self.beam(beam);
                beams.append(&mut next);
            }
        }
        seen.iter()
            .map(|(pos, _)| *pos)
            .collect::<HashSet<Pos>>()
            .len()
    }

    fn entry_points(&self, dir: Direction) -> Vec<Beam> {
        match dir {
            Direction::Up => (0..self.cols as i32)
                .map(|col| ((self.rows as i32 - 1, col), dir))
                .collect(),
            Direction::Down => (0..self.cols as i32)
                .map(|col| ((0i32, col), dir))
                .collect(),
            Direction::Left => (0..self.rows as i32)
                .map(|row| ((row, self.cols as i32 - 1), dir))
                .collect(),
            Direction::Right => (0..self.rows as i32).map(|row| ((row, 0), dir)).collect(),
        }
    }

    fn all_entry_points(&self) -> Vec<Beam> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .map(|dir| self.entry_points(dir))
        .concat()
    }
}

fn read_input() -> Board {
    let mut tiles = HashMap::new();
    let mut cols = 0;
    let mut rows = 0;
    for (row, line) in io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .enumerate()
    {
        cols = line.len();
        rows += 1;
        for (col, c) in line.chars().enumerate() {
            let tile = Tile::from_char(c);
            if Tile::Empty.ne(&tile) {
                tiles.insert((row as i32, col as i32), tile);
            }
        }
    }

    Board { cols, rows, tiles }
}

fn main() {
    let board = read_input();
    let start = ((0, 0), Direction::Right);
    println!("Part 1: {}", board.energized(&start));

    let part2 = board
        .all_entry_points()
        .iter()
        .map(|entry| board.energized(entry))
        .max()
        .unwrap_or(0);
    println!("Part 2: {}", part2)
}
