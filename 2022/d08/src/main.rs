use std::io::{self, BufRead};

struct Forest {
    trees: Vec<Vec<u8>>,
}

impl Forest {
    fn size(&self) -> (usize, usize) {
        let height = self.trees.len();
        match height {
            0 => (0, 0),
            _ => (self.trees[0].len(), height),
        }
    }

    fn height(&self, col: usize, row: usize) -> u8 {
        if row >= self.trees.len() || col >= self.trees[0].len() {
            0
        } else {
            self.trees[row][col]
        }
    }

    fn is_visible(&self, col: usize, row: usize) -> bool {
        let (w, h) = self.size();
        if col == 0 || row == 0 || col == w - 1 || row == h - 1 {
            // edge
            return true;
        }

        let tree = self.height(col, row);
        let row_heights = self.row_heights(row);
        let col_heights = self.col_heights(col);
        let sides = vec![
            &row_heights[0..col],
            &row_heights[col + 1..],
            &col_heights[0..row],
            &col_heights[row + 1..],
        ];
        sides
            .iter()
            .any(|heights| heights.iter().all(|h| *h < tree))
    }

    fn row_heights(&self, row: usize) -> Vec<u8> {
        let row = &self.trees[row];
        row.iter().map(|s| *s).collect()
    }

    fn col_heights(&self, col: usize) -> Vec<u8> {
        self.trees.iter().map(|row| row[col]).collect()
    }

    fn coord_iter(&self) -> CoordIter {
        CoordIter {
            size: self.size(),
            next_coord: (0, 0),
        }
    }

    fn scenic_score(&self, col: usize, row: usize) -> usize {
        let row_heights = self.row_heights(row);
        let col_heights = self.col_heights(col);
        let left = &row_heights[0..=col];
        let right = &row_heights[col..];
        let up = &col_heights[0..=row];
        let down = &col_heights[row..];

        view_distance(left.iter().rev()).unwrap_or_else(|| left.len() - 1)
            * view_distance(up.iter().rev()).unwrap_or_else(|| up.len() - 1)
            * view_distance(down.iter()).unwrap_or_else(|| down.len() - 1)
            * view_distance(right.iter()).unwrap_or_else(|| right.len() - 1)
    }
}

fn view_distance<'a>(mut iter: impl Iterator<Item = &'a u8>) -> Option<usize> {
    let height = *iter.next().unwrap();
    iter.position(|&p| p >= height).map(|p| p + 1)
}

struct CoordIter {
    size: (usize, usize),
    next_coord: (usize, usize),
}

impl Iterator for CoordIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (col, row) = self.next_coord;
        if self.size.0 == 0 || row >= self.size.1 {
            return None;
        }
        self.next_coord = (col + 1, row);
        if self.next_coord.0 == self.size.0 {
            self.next_coord = (0, row + 1)
        }
        Some((col, row))
    }
}

fn read_input() -> Forest {
    Forest {
        trees: io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|line| read_line(&line))
            .collect(),
    }
}

fn read_line(line: &String) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn main() {
    let forest = read_input();

    let visible = forest
        .coord_iter()
        .filter(|&(col, row)| forest.is_visible(col, row))
        .count();
    println!("part1 visible trees: {}", visible);

    let scenic = forest
        .coord_iter()
        .map(|(col, row)| forest.scenic_score(col, row))
        .max()
        .unwrap_or(0);
    println!("part2 max scenic score: {}", scenic);
}
