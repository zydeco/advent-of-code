use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self, BufRead},
    ops::{Add, AddAssign, RangeInclusive},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Pos {
    x: i32,
    y: i32,
}

macro_rules! pos {
    ($a:expr,$b:expr) => {{
        Pos { x: $a, y: $b }
    }};
}

type Board = HashSet<Pos>;

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        pos!(self.x + rhs.x, self.y + rhs.y)
    }
}

fn read_line(y: usize, line: &str) -> Vec<Pos> {
    line.chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(|(x, _)| pos!(x as i32, y as i32))
        .collect()
}

fn read_input() -> Board {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .flat_map(|(idx, line)| read_line(idx, &line))
        .collect()
}

fn round(board: &Board, n: usize) -> Board {
    let mut moves: HashMap<Pos, Pos> = HashMap::new(); // source -> destination
    let mut count_moves: HashMap<Pos, usize> = HashMap::new(); // destination -> # of moves
    for elf in board {
        let dst = propose_move(board, &elf, n).unwrap_or(*elf);
        moves.insert(*elf, dst);
        count_moves.entry(dst).or_default().add_assign(1);
    }

    board
        .iter()
        .map(|elf| {
            let dst = moves.get(elf).unwrap();
            if count_moves.get(dst).unwrap().eq(&1) {
                *dst
            } else {
                *elf
            }
        })
        .collect()
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            dir::N => write!(f, "N"),
            dir::S => write!(f, "S"),
            dir::E => write!(f, "E"),
            dir::W => write!(f, "W"),
            dir::NE => write!(f, "NE"),
            dir::SE => write!(f, "SE"),
            dir::NW => write!(f, "NW"),
            dir::SW => write!(f, "SW"),
            _ => write!(f, "({},{})", self.x, self.y),
        }
    }
}

mod dir {
    use crate::Pos;
    pub const N: Pos = pos!(0, -1);
    pub const S: Pos = pos!(0, 1);
    pub const E: Pos = pos!(1, 0);
    pub const W: Pos = pos!(-1, 0);
    pub const NE: Pos = pos!(1, -1);
    pub const NW: Pos = pos!(-1, -1);
    pub const SE: Pos = pos!(1, 1);
    pub const SW: Pos = pos!(-1, 1);
    pub const ALL: [Pos; 8] = [N, NW, W, SW, S, SE, E, NE];
    pub const CHECK_N: [Pos; 3] = [NE, N, NW];
    pub const CHECK_S: [Pos; 3] = [SE, S, SW];
    pub const CHECK_E: [Pos; 3] = [NE, E, SE];
    pub const CHECK_W: [Pos; 3] = [SW, W, NW];
    pub const CHECK_ALL: [[Pos; 3]; 7] = [
        CHECK_N, CHECK_S, CHECK_W, CHECK_E, CHECK_N, CHECK_S, CHECK_W,
    ];
}

fn propose_move(board: &Board, elf: &Pos, round: usize) -> Option<Pos> {
    if dir::ALL.iter().all(|&d| !board.contains(&elf.add(d))) {
        // all around is empty
        return None;
    }

    // not all empty, maybe move somewhere
    // possible optimization since all places have been checked already
    let directions_to_check = &dir::CHECK_ALL[round % 4..(4 + round % 4)];
    directions_to_check
        .iter()
        .find(|dirs| dirs.iter().all(|&d| !board.contains(&elf.add(d))))
        .map(|dirs| elf.add(dirs[1]))
}

fn pos_range<'a>(values: &mut dyn Iterator<Item = &'a Pos>) -> RangeInclusive<Pos> {
    let mut min = pos!(i32::MAX, i32::MAX);
    let mut max = pos!(i32::MIN, i32::MIN);
    for pos in values {
        if pos.x < min.x {
            min.x = pos.x;
        }
        if pos.x > max.x {
            max.x = pos.x;
        }
        if pos.y < min.y {
            min.y = pos.y;
        }
        if pos.y > max.y {
            max.y = pos.y;
        }
    }
    min..=max
}

fn print_board(board: &Board) {
    print_board_in(board, pos_range(&mut board.iter()));
}

fn print_board_in(board: &Board, range: RangeInclusive<Pos>) {
    let (min, max) = range.into_inner();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let pos = pos!(x, y);
            print!("{}", if board.contains(&pos) { '#' } else { '.' })
        }
        print!("\n")
    }
}

fn empty_tiles(board: &Board) -> usize {
    let (min, max) = pos_range(&mut board.iter()).into_inner();
    let (size_x, size_y) = ((max.x - min.x + 1) as usize, (max.y - min.y + 1) as usize);
    size_x * size_y - board.len()
}

fn main() {
    let input = read_input();
    print_board(&input);

    // part 1
    let mut board = input;
    for r in 0..10 {
        board = round(&board, r);
        println!("== End of Round {} ==", r + 1);
        print_board_in(&board, pos!(-3, -2)..=pos!(10, 9));
        println!("");
    }
    println!("Empty tiles: {}", empty_tiles(&board));

    // part 2
    let mut count = 10;
    loop {
        let new_board = round(&board, count);
        if new_board.eq(&board) {
            break;
        }
        board = new_board;
        count += 1;
    }
    println!("Stopped after {} rounds", count + 1);
}
