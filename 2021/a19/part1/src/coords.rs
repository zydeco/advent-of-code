use std::ops::{Add,Sub,Neg};
use std::fmt::Display;

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[derive(Debug,Copy,Clone)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270
}

#[derive(Debug,Copy,Clone)]
pub enum Facing {
    PlusX,
    MinusX,
    PlusY,
    MinusY,
    PlusZ,
    MinusZ,
}

use Rotation::*;
use Facing::*;

pub const FLIPS: [(Facing,Rotation); 24] = [    
    (PlusX, R0), (PlusX, R90), (PlusX, R180), (PlusX, R270),
    (MinusX, R0), (MinusX, R90), (MinusX, R180), (MinusX, R270),
    (PlusY, R0), (PlusY, R90), (PlusY, R180), (PlusY, R270),
    (MinusY, R0), (MinusY, R90), (MinusY, R180), (MinusY, R270),
    (PlusZ, R0), (PlusZ, R90), (PlusZ, R180), (PlusZ, R270),
    (MinusZ, R0), (MinusZ, R90), (MinusZ, R180), (MinusZ, R270),
];

pub fn inverse_flip(facing: Facing, rotation: Rotation) -> (Facing, Rotation) {
    match (facing, rotation) {
        // +x
        (PlusX, R0) =>   (PlusX, R0),
        (PlusX, R90) =>  (PlusX, R270),
        (PlusX, R180) => (PlusX, R180),
        (PlusX, R270) => (PlusX, R90),
        // -x
        (MinusX, R0) =>   (MinusX, R0),
        (MinusX, R90) =>  (MinusX, R90),
        (MinusX, R180) => (MinusX, R180),
        (MinusX, R270) => (MinusX, R270),
        // +y
        (PlusY, R0) =>   (MinusY, R0),
        (PlusY, R90) =>  (MinusZ, R270),
        (PlusY, R180) => (PlusY, R180),
        (PlusY, R270) => (PlusZ, R0),
        // -y
        (MinusY, R0) =>   (PlusY, R0),
        (MinusY, R90) =>  (MinusZ, R90),
        (MinusY, R180) => (MinusY, R180),
        (MinusY, R270) => (PlusZ, R90),
        // +z
        (PlusZ, R0) =>   (PlusY, R270),
        (PlusZ, R90) =>  (MinusY, R270),
        (PlusZ, R180) => (MinusZ, R0),
        (PlusZ, R270) => (PlusZ, R270),
        // -z
        (MinusZ, R0) =>   (PlusZ, R180),
        (MinusZ, R90) =>  (MinusY, R90),
        (MinusZ, R180) => (MinusZ, R180),
        (MinusZ, R270) => (PlusY, R90),
    }
}

impl Coord {
    pub fn from(x: i32, y: i32, z: i32) -> Coord {
        Coord{x:x, y:y, z:z}
    }

    pub fn from_str(src: &str) -> Option<Coord> {
        let xyz = src.split(',')
            .map(|s| i32::from_str_radix(s, 10))
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        if xyz.len() == 3 {
            return Some(Coord{x:xyz[0], y:xyz[1], z:xyz[2]})
        }
        return None
    }

    pub fn rotate(&self, facing: Facing, rotation: Rotation) -> Coord {
        match (facing, rotation) {
            // +x
            (PlusX, R0) => Coord{x: self.x, y: self.y, z: self.z},
            (PlusX, R90) => Coord{x: self.x, y: -self.z, z: self.y},
            (PlusX, R180) => Coord{x: self.x, y: -self.y, z: -self.z},
            (PlusX, R270) => Coord{x: self.x, y: self.z, z: -self.y},
            // -x
            (MinusX, R0) => Coord{x: -self.x, y: self.z, z: self.y},
            (MinusX, R90) => Coord{x: -self.x, y: -self.z, z: -self.y},
            (MinusX, R180) => Coord{x: -self.x, y: self.y, z: -self.z},
            (MinusX, R270) => Coord{x: -self.x, y: -self.y, z: self.z},
            // +y
            (PlusY, R0) => Coord{x: self.y, y: -self.x, z: self.z},
            (PlusY, R90) => Coord{x: self.y, y: -self.z, z: -self.x},
            (PlusY, R180) => Coord{x: self.y, y: self.x, z: -self.z},
            (PlusY, R270) => Coord{x: self.y, y: self.z, z: self.x},
            // -y
            (MinusY, R0) => Coord{x: -self.y, y: self.x, z: self.z},
            (MinusY, R90) => Coord{x: -self.y, y: self.z, z: -self.x},
            (MinusY, R180) => Coord{x: -self.y, y: -self.x, z: -self.z},
            (MinusY, R270) => Coord{x: -self.y, y: -self.z, z: self.x},
            // +z
            (PlusZ, R0) => Coord{x: self.z, y: self.x, z: self.y},
            (PlusZ, R90) => Coord{x: self.z, y: -self.x, z: -self.y},
            (PlusZ, R180) => Coord{x: self.z, y: self.y, z: -self.x},
            (PlusZ, R270) => Coord{x: self.z, y: -self.y, z: self.x},
            // -z
            (MinusZ, R0) => Coord{x: -self.z, y: self.y, z: self.x},
            (MinusZ, R90) => Coord{x: -self.z, y: -self.x, z: self.y},
            (MinusZ, R180) => Coord{x: -self.z, y: -self.y, z: -self.x},
            (MinusZ, R270) => Coord{x: -self.z, y: self.x, z: -self.y},
        }
    }

    pub fn manhattan_distance(&self, other: &Coord) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u32
    }
}

impl Default for Coord {
    fn default() -> Coord {
        Coord::from(0,0,0)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, other: &Coord) -> Coord {
        Coord{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, other: &Coord) -> Coord {
        Coord{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Coord {
        Coord{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Neg for &Coord {
    type Output = Coord;

    fn neg(self) -> Coord {
        Coord{x: -self.x, y: -self.y, z: -self.z}
    }
}


impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{},{},{}", self.x, self.y, self.z)?;
        Ok(())
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            R0 => "r0",
            R90 => "r90",
            R180 => "r180",
            R270 => "r270"
        })?;
        Ok(())
    }
}

impl Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            PlusX => "+X",
            MinusX => "-X",
            PlusY => "+Y",
            MinusY => "-Y",
            PlusZ => "+Z",
            MinusZ => "-Z"
        })?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::coords::{Coord,FLIPS,inverse_flip};

    #[test]
    fn inverse_flips() {
        let c1 = Coord::from(1,2,3);
        for (facing,rotation) in FLIPS {
            let c2 = c1.rotate(facing, rotation);
            let (f2, r2) = inverse_flip(facing, rotation);
            let c3 = c2.rotate(f2, r2);
            assert_eq!(c3, c1, "{} {}{} -> {} {}{} -> {}", c1, facing, rotation, c2, f2, r2, c3);
        }
    }
}