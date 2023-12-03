use std::{
    collections::HashMap,
    io::{self, BufRead},
};

type Coord = (i32, i32); // col, line

#[derive(Debug)]
enum Glyph {
    Number(u32),
    Symbol(char),
}

fn read_line(line_number: i32, str: String) -> HashMap<Coord, Glyph> {
    let mut items: HashMap<Coord, Glyph> = HashMap::new();
    for (col, c) in str.chars().enumerate() {
        let col = col as i32;
        let coord = (col, line_number);
        if let Some(value) = c.to_digit(10) {
            let prev_coord = (col - 1, line_number);
            if let Some(Glyph::Number(prev)) = items.get(&prev_coord) {
                // existing digit
                items.insert(coord, Glyph::Number(value + prev * 10));
                items.remove(&prev_coord);
            } else {
                // new digit
                items.insert(coord, Glyph::Number(value));
            }
        } else if c != '.' {
            items.insert(coord, Glyph::Symbol(c));
        }
    }
    items
}

impl Glyph {
    fn width(&self) -> i32 {
        match self {
            Glyph::Number(n) => n.to_string().len() as i32,
            Glyph::Symbol(_) => 1,
        }
    }

    fn adjacent_coords(&self, coord: Coord) -> Vec<Coord> {
        let mut coords = vec![];
        let left = coord.0 - self.width();
        let right = coord.0 + 1;
        let top = coord.1 - 1;
        let bottom = coord.1 + 1;
        for col in left..=right {
            coords.push((col, top));
            coords.push((col, bottom));
        }
        coords.push((left, coord.1));
        coords.push((right, coord.1));
        coords
    }

    fn adjacent_numbers(&self, coord: Coord, map: &HashMap<Coord, Glyph>) -> Vec<u32> {
        let mut numbers = vec![];
        let left: i32 = coord.0 - self.width();
        let right = coord.0 + 1;
        let top = coord.1 - 1;
        let bottom = coord.1 + 1;
        for col in left..=right {
            if let Some(Glyph::Number(n)) = map.get(&(col, top)) {
                numbers.push(*n);
            }
            if let Some(Glyph::Number(n)) = map.get(&(col, bottom)) {
                numbers.push(*n);
            }
        }
        if let Some(Glyph::Number(n)) = map.get(&(left, coord.1)) {
            numbers.push(*n);
        }
        if let Some(Glyph::Number(n)) = map.get(&(right, coord.1)) {
            numbers.push(*n);
        }
        // 2 or 3 digit numbers further to the right
        for line in top..=bottom {
            if let Some(Glyph::Number(n)) = map.get(&(right + 1, line)) {
                if *n > 9 {
                    numbers.push(*n);
                }
            }
            if let Some(Glyph::Number(n)) = map.get(&(right + 2, line)) {
                if *n > 99 {
                    numbers.push(*n);
                }
            }
        }

        numbers
    }

    fn is_part_number(&self, coord: Coord, map: &HashMap<Coord, Glyph>) -> bool {
        self.part_number_value(coord, map).is_some()
    }

    fn part_number_value(&self, coord: Coord, map: &HashMap<Coord, Glyph>) -> Option<u32> {
        if let Glyph::Number(value) = self {
            // check adjacents
            for c in self.adjacent_coords(coord) {
                if let Some(Glyph::Symbol(_)) = map.get(&c) {
                    return Some(*value);
                }
            }
        }
        None
    }

    fn gear_ratio(&self, coord: Coord, map: &HashMap<Coord, Glyph>) -> Option<u32> {
        if let Glyph::Symbol('*') = self {
            let adjacent_numbers = self.adjacent_numbers(coord, map);
            if adjacent_numbers.len() == 2 {
                return Some(adjacent_numbers[0] * adjacent_numbers[1]);
            }
        }
        None
    }
}

fn pretty_print(map: &HashMap<Coord, Glyph>) {
    let cols = map.keys().map(|k| k.0).max().unwrap();
    let rows = map.keys().map(|k| k.1).max().unwrap();
    for row in 0..=rows {
        for col in 0..=cols {
            let coord = (col, row);
            if let Some(glyph) = map.get(&coord) {
                match glyph {
                    Glyph::Number(n) => {
                        for _ in 1..glyph.width() {
                            // backspace to print multi-digit number
                            print!("\x08");
                        }
                        if glyph.is_part_number(coord, map) {
                            // part number: green
                            print!("\x1B[42m{}\x1B[0m", n);
                        } else {
                            // not a part number: red
                            print!("\x1B[41m{}\x1B[0m", n);
                        }
                    }
                    Glyph::Symbol(v) => {
                        if glyph.gear_ratio(coord, map).is_some() {
                            // gear: yellow
                            print!("\x1B[43m{}\x1B[0m", v);
                        } else {
                            // symbol: cyan
                            print!("\x1B[46m{}\x1B[0m", v);
                        }
                    }
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let map: HashMap<Coord, Glyph> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .flat_map(|(n, s)| read_line(n as i32, s))
        .collect();

    pretty_print(&map);

    let part_number_sum: u32 = map
        .iter()
        .flat_map(|(&k, v)| v.part_number_value(k, &map))
        .sum();
    println!("Part number sum: {}", part_number_sum);

    let gear_ratio_sum: u32 = map.iter().flat_map(|(&k, v)| v.gear_ratio(k, &map)).sum();
    println!("Gear ratio sum: {}", gear_ratio_sum);
}
