use std::{
    fmt::Display,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rocks,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => panic!("Bad tile “{}”", c),
        }
    }
}

struct Map {
    rows: Vec<Vec<Tile>>,
}

impl Map {
    fn transposed(&self) -> Self {
        let mut rows = vec![];
        for col in 0..self.rows[0].len() {
            rows.push(self.col(col));
        }
        Map { rows }
    }

    fn col(&self, idx: usize) -> Vec<Tile> {
        if idx > self.rows[0].len() {
            panic!("col out of range");
        }
        self.rows.iter().map(|row| row[idx]).collect()
    }

    // mirrors the first n rows
    fn has_mirror_h(&self, n: usize) -> bool {
        assert!(n > 0 && n < self.rows.len());
        let mut a = self.rows.iter().take(n).rev();
        let mut b = self.rows.iter().skip(n);

        while let (Some(row_a), Some(row_b)) = (a.next(), b.next()) {
            if row_a.ne(row_b) {
                return false;
            }
        }
        return true;
    }

    fn mirrored_rows(&self, n: usize) -> usize {
        assert!(n > 0 && n < self.rows.len());
        let mut a = self.rows.iter().take(n).rev();
        let mut b = self.rows.iter().skip(n);
        let mut n = 0;
        while let (Some(row_a), Some(row_b)) = (a.next(), b.next()) {
            if row_a.eq(row_b) {
                n += 1;
            }
        }
        n
    }

    fn has_smudge_for_row(&self, n: usize) -> bool {
        let mut a = self.rows.iter().take(n).rev();
        let mut b = self.rows.iter().skip(n);
        while let (Some(row_a), Some(row_b)) = (a.next(), b.next()) {
            if row_a.ne(row_b) {
                // there must only be one difference
                let diffs = row_a
                    .iter()
                    .zip(row_b.iter())
                    .filter(|(tile_a, tile_b)| tile_a.ne(tile_b))
                    .count();
                if diffs == 1 {
                    return true;
                }
            }
        }
        false
    }

    fn has_mirror_h_smudged(&self, n: usize) -> bool {
        assert!(n > 0 && n < self.rows.len());
        if self.has_mirror_h(n) {
            return false;
        }
        let num_rows = self.rows.len();
        let expected_mirrored_rows = if n <= num_rows / 2 {
            n - 1
        } else {
            num_rows - n - 1
        };
        if self.mirrored_rows(n) == expected_mirrored_rows {
            // find mismatch
            return self.has_smudge_for_row(n);
        }
        false
    }

    fn mirror_h(&self) -> Option<usize> {
        (1..self.rows.len())
            .filter(|n| self.has_mirror_h(*n))
            .next()
    }

    fn mirror(&self) -> usize {
        self.mirror_h()
            .map(|h| h * 100)
            .or_else(|| self.transposed().mirror_h())
            .unwrap()
    }

    fn mirror_h_smudged(&self) -> Option<usize> {
        (1..self.rows.len())
            .filter(|n| self.has_mirror_h_smudged(*n))
            .next()
    }

    fn mirror_smudged(&self) -> usize {
        self.mirror_h_smudged()
            .map(|h| h * 100)
            .or_else(|| self.transposed().mirror_h_smudged())
            .unwrap()
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ash => '.',
                Self::Rocks => '#',
            }
        )
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn read_input() -> Vec<Map> {
    let mut maps = vec![];
    let mut rows = vec![];
    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        if line.len() == 0 {
            // new map
            maps.push(Map { rows });
            rows = vec![];
        } else {
            // row
            rows.push(line.chars().map(Tile::from_char).collect());
        }
    }

    // last map
    if rows.len() > 0 {
        maps.push(Map { rows });
    }

    maps
}
fn main() {
    let maps = read_input();
    let mirrors: usize = maps.iter().map(Map::mirror).sum();
    println!("Part 1: {}", mirrors);

    let mirrors_smudged: usize = maps.iter().map(Map::mirror_smudged).sum();
    println!("Part 2: {}", mirrors_smudged);
}
