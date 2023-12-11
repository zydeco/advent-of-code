use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use combinatorial::Combinations;

type Pos = (usize, usize); // row, col

struct Map {
    map: Vec<Vec<bool>>, // row, col
}

fn abs_dif(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn manhattan_distance(a: Pos, b: Pos) -> usize {
    abs_dif(a.0, b.0) + abs_dif(a.1, b.1)
}

fn count_between(values: &HashSet<usize>, a: usize, b: usize) -> usize {
    let range = a.min(b)..a.max(b);
    values.iter().filter(|&value| range.contains(value)).count()
}

impl Map {
    fn get(&self, pos: Pos) -> bool {
        self.map
            .get(pos.0)
            .and_then(|row| row.get(pos.1))
            .map(|x| *x)
            .unwrap_or(false)
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn empty_cols(&self) -> HashSet<usize> {
        let width = self.width();
        let mut empty_cols: HashSet<usize> = HashSet::from_iter(0..width);
        for row in self.map.iter() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value == true {
                    empty_cols.remove(&col_idx);
                }
            }
        }
        empty_cols
    }

    fn empty_rows(&self) -> HashSet<usize> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if row.iter().all(|&x| x == false) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    fn expand_row(row: &Vec<bool>, empty_cols: &HashSet<usize>) -> Vec<bool> {
        let mut new_row = vec![];
        for (idx, &value) in row.iter().enumerate() {
            new_row.push(value);
            if empty_cols.contains(&idx) {
                assert_eq!(value, false);
                new_row.push(value);
            }
        }
        new_row
    }

    fn expand(&self) -> Map {
        let empty_cols: HashSet<usize> = self.empty_cols();
        let mut map = Vec::new();
        for row in self.map.iter() {
            map.push(Map::expand_row(row, &empty_cols));
            if row.iter().all(|&value| value == false) {
                map.push(Map::expand_row(row, &empty_cols));
            }
        }

        Map { map }
    }

    fn galaxies(&self) -> Vec<Pos> {
        let mut galaxies = vec![];
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value == true {
                    galaxies.push((row_idx, col_idx));
                }
            }
        }
        galaxies
    }

    fn distances_sum(&self) -> usize {
        let galaxies = self.galaxies();
        Combinations::of_size(galaxies, 2)
            .map(|pair| manhattan_distance(pair[0], pair[1]))
            .sum()
    }

    fn distances_sum_expanded(&self, expansion: usize) -> usize {
        let galaxies = self.galaxies();
        let empty_rows = self.empty_rows();
        let empty_cols = self.empty_cols();
        Combinations::of_size(galaxies, 2)
            .map(|pair| {
                manhattan_distance(pair[0], pair[1])
                    + (expansion - 1)
                        * (count_between(&empty_rows, pair[0].0, pair[1].0)
                            + count_between(&empty_cols, pair[0].1, pair[1].1))
            })
            .sum()
    }
}

fn read_input() -> Map {
    Map {
        map: io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
    }
}

fn main() {
    let map = read_input();
    let expanded = map.expand();
    let distances1 = expanded.distances_sum();
    println!("Part 1: {}", distances1);
    let distances2 = map.distances_sum_expanded(1000000);
    println!("Part 2: {}", distances2);
}
