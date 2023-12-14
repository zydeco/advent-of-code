use std::{
    fmt::Display,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
    Cube,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Rock => 'O',
                Self::Cube => '#',
            }
        )
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Rock,
            '#' => Self::Cube,
            _ => panic!("Invalid tile “{}”", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    cols: Vec<Vec<Tile>>,
}

fn fall_back(row: &mut Vec<Tile>) {
    for slice in row.split_mut(|t| Tile::Cube.eq(t)) {
        let rocks = slice.iter().filter(|t| Tile::Rock.eq(t)).count();
        slice[0..rocks].fill(Tile::Rock);
        slice[rocks..].fill(Tile::Empty);
    }
}

fn fall_forward(row: &mut Vec<Tile>) {
    for slice in row.split_mut(|t| Tile::Cube.eq(t)) {
        let rocks = slice.iter().filter(|t| Tile::Rock.eq(t)).count();
        let len = slice.len();
        slice[0..(len - rocks)].fill(Tile::Empty);
        slice[(len - rocks)..].fill(Tile::Rock);
    }
}

fn fall_back_ref(row: &mut Vec<&mut Tile>) {
    for slice in row.split_mut(|t| Tile::Cube.eq(t)) {
        let rocks = slice.iter().filter(|t| Tile::Rock.eq(t)).count();
        for i in 0..rocks {
            *(slice[i]) = Tile::Rock;
        }
        for i in rocks..slice.len() {
            *(slice[i]) = Tile::Empty;
        }
    }
}

fn fall_forward_ref(row: &mut Vec<&mut Tile>) {
    for slice in row.split_mut(|t| Tile::Cube.eq(t)) {
        let rocks = slice.iter().filter(|t| Tile::Rock.eq(t)).count();
        let len = slice.len();
        for i in 0..(len - rocks) {
            *(slice[i]) = Tile::Empty;
        }
        for i in (len - rocks)..len {
            *(slice[i]) = Tile::Rock;
        }
    }
}

impl Map {
    fn transposed(&self) -> Self {
        let mut cols = vec![];
        for row in 0..self.cols[0].len() {
            cols.push(self.row(row));
        }
        Map { cols }
    }

    fn row(&self, idx: usize) -> Vec<Tile> {
        if idx > self.cols[0].len() {
            panic!("row out of range");
        }
        self.cols.iter().map(|col| col[idx]).collect()
    }

    fn row_ref(&mut self, idx: usize) -> Vec<&mut Tile> {
        if idx > self.cols[0].len() {
            panic!("row out of range");
        }
        self.cols.iter_mut().map(|col| &mut col[idx]).collect()
    }

    fn fall_north(&mut self) {
        for i in 0..self.cols.len() {
            fall_back(&mut self.cols[i]);
        }
    }

    fn fall_south(&mut self) {
        for i in 0..self.cols.len() {
            fall_forward(&mut self.cols[i]);
        }
    }

    fn fall_east(&mut self) {
        for i in 0..self.cols[0].len() {
            fall_forward_ref(&mut self.row_ref(i));
        }
    }

    fn fall_west(&mut self) {
        for i in 0..self.cols[0].len() {
            fall_back_ref(&mut self.row_ref(i));
        }
    }

    fn load(&self) -> usize {
        self.cols
            .iter()
            .map(|col| {
                col.iter()
                    .rev()
                    .enumerate()
                    .filter(|(_, tile)| Tile::Rock.eq(tile))
                    .map(|(idx, _)| idx + 1)
                    .sum::<usize>()
            })
            .sum()
    }

    fn cycle(&mut self) {
        self.fall_north();
        self.fall_west();
        self.fall_south();
        self.fall_east();
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.transposed().cols {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_input() -> Map {
    let cols = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();
    Map { cols }.transposed()
}

fn main() {
    let map = read_input();
    println!("{}", map);

    let mut map1 = map.clone();
    map1.fall_north();
    println!("Load: {}", map1.load());
}
