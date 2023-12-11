use std::{
    fmt::Display,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Pipe {
    fn goes(&self, dir: Direction) -> bool {
        match (self, dir) {
            (Pipe::Vertical, Direction::North | Direction::South) => true,
            (Pipe::Horizontal, Direction::East | Direction::West) => true,
            (Pipe::NorthEast, Direction::North | Direction::East) => true,
            (Pipe::NorthWest, Direction::North | Direction::West) => true,
            (Pipe::SouthWest, Direction::South | Direction::West) => true,
            (Pipe::SouthEast, Direction::South | Direction::East) => true,
            _ => false,
        }
    }

    fn from_to(a: Direction, b: Direction) -> Self {
        match (a, b) {
            (Direction::North, Direction::East) | (Direction::East, Direction::North) => {
                Self::NorthEast
            }
            (Direction::North, Direction::West) | (Direction::West, Direction::North) => {
                Self::NorthWest
            }
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
                Self::Vertical
            }
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => {
                Self::Horizontal
            }
            (Direction::South, Direction::East) | (Direction::East, Direction::South) => {
                Self::SouthEast
            }
            (Direction::South, Direction::West) | (Direction::West, Direction::South) => {
                Self::SouthWest
            }
            _ => panic!("Invalid pipe from {:?} to {:?}", a, b),
        }
    }

    fn directions(&self) -> [Direction; 2] {
        match self {
            Pipe::Vertical => [Direction::North, Direction::South],
            Pipe::Horizontal => [Direction::East, Direction::West],
            Pipe::NorthEast => [Direction::North, Direction::East],
            Pipe::NorthWest => [Direction::North, Direction::West],
            Pipe::SouthWest => [Direction::South, Direction::West],
            Pipe::SouthEast => [Direction::South, Direction::East],
        }
    }

    // next direction entering from certain direction
    // eg entering horizontal pipe while moving east will continue east
    fn next(&self, moving_in: Direction) -> Direction {
        match (self, moving_in) {
            (Pipe::Vertical, Direction::South) => Direction::South,
            (Pipe::Vertical, Direction::North) => Direction::North,
            (Pipe::Horizontal, Direction::East) => Direction::East,
            (Pipe::Horizontal, Direction::West) => Direction::West,
            (Pipe::NorthEast, Direction::South) => Direction::East,
            (Pipe::NorthEast, Direction::West) => Direction::North,
            (Pipe::NorthWest, Direction::South) => Direction::West,
            (Pipe::NorthWest, Direction::East) => Direction::North,
            (Pipe::SouthWest, Direction::North) => Direction::West,
            (Pipe::SouthWest, Direction::East) => Direction::South,
            (Pipe::SouthEast, Direction::North) => Direction::East,
            (Pipe::SouthEast, Direction::West) => Direction::South,
            _ => panic!("Cannot enter {:?} moving {:?}", self, moving_in),
        }
    }
}

type Pos = (usize, usize); // row, col

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            'S' | '.' => None,
            _ => panic!("Invalid tile {}", c),
        }
    }
}
struct Map {
    pipes: Vec<Vec<Option<Pipe>>>, // row, col
    start: Pos,
}

fn checked_get_2d(array: &Vec<Vec<Option<Pipe>>>, row: usize, col: usize) -> Option<Pipe> {
    array
        .get(row)
        .and_then(|row| row.get(col))
        .map(|x| *x)
        .unwrap_or(None)
}

fn read_input() -> Map {
    let lines = io::stdin().lock().lines().filter_map(Result::ok);
    let mut pipes: Vec<Vec<Option<Pipe>>> = vec![];
    let mut start = (0, 0);
    for (row_num, line) in lines.enumerate() {
        if let Some(start_col) = line.find('S') {
            start = (row_num, start_col);
        }
        let row = line.chars().map(Pipe::from_char).collect();
        pipes.push(row);
    }
    // figure out tile under start
    let tile_north = start
        .0
        .checked_sub(1)
        .and_then(|row| checked_get_2d(&pipes, row, start.1));
    let tile_south = checked_get_2d(&pipes, start.0 + 1, start.1);
    let tile_east = checked_get_2d(&pipes, start.0, start.1 + 1);
    let tile_west = start
        .1
        .checked_sub(1)
        .and_then(|col| checked_get_2d(&pipes, start.0, col));
    let north = tile_north
        .map(|tile| tile.goes(Direction::South))
        .unwrap_or(false);
    let south = tile_south
        .map(|tile| tile.goes(Direction::North))
        .unwrap_or(false);
    let east = tile_east
        .map(|tile| tile.goes(Direction::West))
        .unwrap_or(false);
    let west = tile_west
        .map(|tile| tile.goes(Direction::East))
        .unwrap_or(false);
    let start_tile = &mut pipes[start.0][start.1];
    if north && south {
        *start_tile = Some(Pipe::Vertical);
    } else if north && east {
        *start_tile = Some(Pipe::NorthEast);
    } else if north && west {
        *start_tile = Some(Pipe::NorthWest);
    } else if south && east {
        *start_tile = Some(Pipe::SouthEast);
    } else if south && west {
        *start_tile = Some(Pipe::SouthWest);
    } else if west && east {
        *start_tile = Some(Pipe::Horizontal);
    } else {
        panic!("Don't know how to start!");
    }

    Map { pipes, start }
}

impl Direction {
    fn from(&self, pos: Pos) -> Option<Pos> {
        match self {
            Direction::North => pos.0.checked_sub(1).map(|new0| (new0, pos.1)),
            Direction::East => Some((pos.0, pos.1 + 1)),
            Direction::South => Some((pos.0 + 1, pos.1)),
            Direction::West => pos.1.checked_sub(1).map(|new1| (pos.0, new1)),
        }
    }
}

impl Map {
    fn get(&self, pos: Pos) -> Option<Pipe> {
        self.pipes
            .get(pos.0)
            .and_then(|row| row.get(pos.1))
            .map(|x| *x)
            .unwrap_or(None)
    }

    fn next(&self, moving_in: Direction, from: Pos) -> Option<(Direction, Pos)> {
        if let Some(next_pos) = moving_in.from(from) {
            if let Some(next_pipe) = self.get(next_pos) {
                return Some((next_pipe.next(moving_in), next_pos));
            }
        }
        return None;
    }

    fn enclosed_tiles(&self) -> usize {
        0
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.pipes.iter() {
            for pipe in row {
                if let Some(p) = pipe {
                    write!(f, "{}", p)?
                } else {
                    write!(f, ".")?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Horizontal => '═',
                Pipe::Vertical => '║',
                Pipe::NorthEast => '╚',
                Pipe::NorthWest => '╝',
                Pipe::SouthEast => '╔',
                Pipe::SouthWest => '╗',
            }
        )
    }
}

fn main() {
    let map = read_input();
    println!("{}", map);
    let mut directions = map.get(map.start).map(|pipe| pipe.directions()).unwrap();
    let mut pos0 = map.start;
    let mut pos1 = map.start;
    let mut steps = 0;
    // follow until reaching the same point
    while steps == 0 || pos0 != pos1 {
        /*println!(
            "Step {}:\n  Moving {:?} from {:?}\n  Moving {:?} from {:?}",
            steps, directions[0], pos0, directions[1], pos1
        );*/
        steps += 1;

        (directions[0], pos0) = map.next(directions[0], pos0).unwrap();
        (directions[1], pos1) = map.next(directions[1], pos1).unwrap();
    }
    println!("Steps: {}", steps);

    println!("Enclosed tiles: {}", map.enclosed_tiles());
}
