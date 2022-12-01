use std::fmt::Write;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fmt::Display;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
#[repr(u8)]
enum Herd {
    East,
    South,
}

type Coord = (u8,u8);

#[derive(Debug,Clone,PartialEq,Eq)]
struct Seabed {
    size: Coord,
    cucumbers: HashMap<Coord,Herd>
}

impl Herd {
    fn from_char(c: char) -> Option<Herd> {
        match c {
            '>' => Some(Herd::East),
            'v' => Some(Herd::South),
            _ => None
        }
    }
}

impl Seabed {
    fn step(&self) -> Seabed {
        let mut cucumbers = HashMap::with_capacity(self.cucumbers.len());
        let east = self.herd(Herd::East);
        for coord in east {
            let mut next = ((coord.0 + 1) % self.size.0, coord.1);
            if self.cucumbers.contains_key(&next) {
                next = coord;
            }
            cucumbers.insert(next, Herd::East);
        }

        let south = self.herd(Herd::South);
        for coord in south {
            let mut next = (coord.0, (coord.1 + 1) % self.size.1);
            if cucumbers.contains_key(&next) || self.cucumbers.get(&next) == Some(&Herd::South) {
                next = coord;
            }
            cucumbers.insert(next, Herd::South);
        }
        Seabed{size: self.size, cucumbers: cucumbers}
    }

    fn herd(&self, herd: Herd) -> Vec<(u8,u8)> {
        self.cucumbers.iter().filter(|(_,&v)| herd == v).map(|(&k,_)| k).collect()
    }
}

struct SeabedIterator {
    state: Seabed,
}

impl Iterator for SeabedIterator {
    type Item = Seabed;
    
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.state.step();
        if next == self.state {
            return None;
        } else {
            self.state = next;
            return Some(self.state.clone());
        }
    }
}

impl IntoIterator for Seabed {
    type Item = Seabed;
    type IntoIter = SeabedIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        SeabedIterator{state: self.clone()}
    }
}

impl Display for Seabed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut line = String::with_capacity(self.size.0 as usize);
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let coord = (x as u8, y as u8);
                line.write_char(match (cfg!(feature="emoji"),self.cucumbers.get(&coord)) {
                    (true,Some(_)) => '🪱',
                    (true,None) => '🟦',
                    (false,Some(Herd::East)) => '>',
                    (false,Some(Herd::South)) => 'v',
                    (false,None) => '.'
                })?;
            }
            writeln!(f, "{}", line)?;
            line.clear();
        }
        Ok(())
    }
}

fn read_input() -> Seabed {
    let mut cucumbers = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y,line) in io::stdin().lock().lines().filter_map(Result::ok).enumerate() {
        width = line.len();
        height = y+1;
        for (x,c) in line.chars().enumerate() {
            let coord = (x as u8, y as u8);
            if let Some(herd) = Herd::from_char(c) {
                cucumbers.insert(coord, herd);
            }
        }
    }
    Seabed{size:(width as u8,height as u8), cucumbers: cucumbers}
}

#[inline]
fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let input = read_input();
    println!("Initial state:\n{}", input);
    
    let mut step = 0;
    for state in input {
        step += 1;
        cls();
        println!("After {} step{}:\n{}", step, if step == 1 { "" } else { "s" }, state);
        std::thread::sleep(std::time::Duration::from_millis(33));
    }

    println!("They stop moving after {} steps", step+1);
}
