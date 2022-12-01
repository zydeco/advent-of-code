use std::ops::RangeInclusive;
use std::io::{self,BufRead};

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Cuboid {
    // max is exclusive
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl std::fmt::Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "x={}..{},y={}..{},z={}..{}", self.x_min, self.x_max-1, self.y_min, self.y_max-1, self.z_min, self.z_max-1)?;
        Ok(())
    }
}

#[derive(Debug,Copy,Clone)]
struct Instruction { // on x=10..12,y=10..12,z=10..12
    state: bool,
    cuboid: Cuboid,
}

impl Cuboid {
    fn from_coords(coords: &[i32]) -> Cuboid {
        assert_eq!(6, coords.len(), "Invalid coords length");
        Cuboid{
            x_min: coords[0],
            x_max: coords[1],
            y_min: coords[2],
            y_max: coords[3],
            z_min: coords[4],
            z_max: coords[5],
        }
    }

    fn from_coords_inclusive(coords: &[i32]) -> Cuboid {
        Self::from_coords(&[
            coords[0],
            coords[1]+1,
            coords[2],
            coords[3]+1,
            coords[4],
            coords[5]+1]
        )
    }
    
    fn x_range(&self) -> RangeInclusive<i32> {
        self.x_min..=self.x_max-1
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        self.y_min..=self.y_max-1
    }

    fn z_range(&self) -> RangeInclusive<i32> {
        self.z_min..=self.z_max-1
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        ranges_intersect(self.x_range(), other.x_range()) &&
        ranges_intersect(self.y_range(), other.y_range()) &&
        ranges_intersect(self.z_range(), other.z_range())
    }

    fn volume(&self) -> i64 {
        if !self.has_volume() {
            return 0;
        }
        let x = self.x_max - self.x_min;
        let y = self.y_max - self.y_min;
        let z = self.z_max - self.z_min;
        (x as i64)*(y as i64)*(z as i64)
    }

    fn has_volume(&self) -> bool {
        self.x_max > self.x_min && 
        self.y_max > self.y_min && 
        self.z_max > self.z_min 
    }

    fn intersect(&self, other: &Cuboid) -> Cuboid {
        Cuboid::from_coords(&[
            self.x_min.max(other.x_min),
            self.x_max.min(other.x_max),
            self.y_min.max(other.y_min),
            self.y_max.min(other.y_max),
            self.z_min.max(other.z_min),
            self.z_max.min(other.z_max),
        ])
    }

    fn remove(&self, other: &Cuboid) -> Vec<Cuboid> {
        // return non-intersecting sub-cuboids that don't intersect other
        let cuboid = self.intersect(other);
        if !cuboid.has_volume() {
            return vec![*self];
        } else if cuboid == *self {
            return vec![];
        }
        let subcuboids = [
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, self.y_min, cuboid.y_min, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, self.y_min, cuboid.y_min, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, self.y_min, cuboid.y_min, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, cuboid.y_min, cuboid.y_max, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, cuboid.y_min, cuboid.y_max, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, cuboid.y_min, cuboid.y_max, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, cuboid.y_max, self.y_max, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, cuboid.y_max, self.y_max, self.z_min, cuboid.z_min]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, cuboid.y_max, self.y_max, self.z_min, cuboid.z_min]),
            
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, self.y_min, cuboid.y_min, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, self.y_min, cuboid.y_min, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, self.y_min, cuboid.y_min, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, cuboid.y_min, cuboid.y_max, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, cuboid.y_min, cuboid.y_max, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, cuboid.y_max, self.y_max, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, cuboid.y_max, self.y_max, cuboid.z_min, cuboid.z_max]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, cuboid.y_max, self.y_max, cuboid.z_min, cuboid.z_max]),
            
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, self.y_min, cuboid.y_min, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, self.y_min, cuboid.y_min, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, self.y_min, cuboid.y_min, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, cuboid.y_min, cuboid.y_max, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, cuboid.y_min, cuboid.y_max, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, cuboid.y_min, cuboid.y_max, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[self.x_min, cuboid.x_min, cuboid.y_max, self.y_max, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[cuboid.x_min, cuboid.x_max, cuboid.y_max, self.y_max, cuboid.z_max, self.z_max]),
            Cuboid::from_coords(&[cuboid.x_max, self.x_max, cuboid.y_max, self.y_max, cuboid.z_max, self.z_max]),
        ];
        subcuboids.into_iter().filter(Self::has_volume).collect::<Vec<_>>()
    }
}

fn ranges_intersect<T: PartialOrd>(r1: RangeInclusive<T>, r2: RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) || r1.contains(r2.end()) || r2.contains(r1.start()) || r2.contains(r1.end())
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let state = s.starts_with("on ");
        let coords = s.split(|c: char| c != '-' && !c.is_ascii_digit() )
        .map(|s| i32::from_str_radix(s, 10) )
        .filter(Result::is_ok)
        .map(Result::ok)
        .map(Option::unwrap)
        .collect::<Vec<_>>();
        Instruction{
            state: state,
            cuboid: Cuboid::from_coords_inclusive(&coords),
        }
    }

    fn apply(&self, prev_state: &Vec<Cuboid>) -> Vec<Cuboid> {
        let mut state = Vec::with_capacity(prev_state.capacity());
        let part = self.cuboid;
        for cuboid in prev_state {
            if cuboid.intersects(&part) {
                let mut non_intersecting = cuboid.remove(&part);
                state.append(&mut non_intersecting);
            } else {
                state.push(*cuboid);
            }
        }
        if self.state {
            state.push(part);
        }
        state
    }
}

fn read_input() -> Vec<Instruction> {
    io::stdin().lock().lines()
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .filter(|s| s.len() > 0 )
    .map(|s| Instruction::from_str(&s))
    .collect()
}

fn cuboids_on(state: &Vec<Cuboid>) -> i64 {
    state.iter().fold(0, |acc, q| acc + q.volume())
}

fn main() {
    let input = read_input();
    let state = input.iter().fold(vec![], |acc,i| {
        return i.apply(&acc)
    });
    println!("Total: {}", cuboids_on(&state));
    println!("There's {} cuboids now", state.len());
}
