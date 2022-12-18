use std::{
    collections::HashSet,
    hash::Hash,
    io::{self, BufRead},
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos<T> {
    x: T,
    y: T,
    z: T,
}

trait PosValue:
    Sized
    + Hash
    + Eq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + From<i8>
    + TryInto<usize>
    + Copy
    + Ord
{
}

impl<
        T: Sized
            + Hash
            + Eq
            + Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + From<i8>
            + TryInto<usize>
            + Copy
            + Ord,
    > PosValue for T
{
}

impl<T> FromStr for Pos<T>
where
    T: FromStr,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(',')
            .filter_map(|w| w.parse::<T>().ok())
            .collect::<Vec<_>>();
        assert_eq!(coords.len(), 3);
        let mut iter = coords.into_iter();
        Ok(Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        })
    }
}

impl<T> Pos<T>
where
    T: PosValue,
{
    fn up(&self) -> Pos<T> {
        Pos {
            x: self.x,
            y: self.y + 1.into(),
            z: self.z,
        }
    }

    fn down(&self) -> Pos<T> {
        Pos {
            x: self.x,
            y: self.y - 1.into(),
            z: self.z,
        }
    }

    fn left(&self) -> Pos<T> {
        Pos {
            x: self.x - 1.into(),
            y: self.y,
            z: self.z,
        }
    }

    fn right(&self) -> Pos<T> {
        Pos {
            x: self.x + 1.into(),
            y: self.y,
            z: self.z,
        }
    }

    fn front(&self) -> Pos<T> {
        Pos {
            x: self.x,
            y: self.y,
            z: self.z + 1.into(),
        }
    }

    fn back(&self) -> Pos<T> {
        Pos {
            x: self.x,
            y: self.y,
            z: self.z - 1.into(),
        }
    }

    fn neighbours(&self) -> [Pos<T>; 6] {
        [
            self.up(),
            self.down(),
            self.left(),
            self.right(),
            self.front(),
            self.back(),
        ]
    }

    fn num_exposed_faces(&self, world: &HashSet<Pos<T>>) -> usize {
        self.neighbours()
            .iter()
            .filter(|nb| !world.contains(nb))
            .count()
    }
}

fn read_input() -> HashSet<Pos<i16>> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|ln| Pos::from_str(&ln).unwrap())
        .collect()
}

fn surface_area<T: PosValue>(shape: &HashSet<Pos<T>>) -> usize {
    shape.iter().map(|b| b.num_exposed_faces(shape)).sum()
}

#[derive(Debug)]
struct Cube<T: PosValue> {
    min: Pos<T>,
    max: Pos<T>,
}

impl<T: PosValue> Cube<T> {
    fn containing(shape: &HashSet<Pos<T>>) -> Cube<T> {
        let min = Pos {
            x: shape.iter().map(|b| b.x).min().unwrap() - 1.into(),
            y: shape.iter().map(|b| b.y).min().unwrap() - 1.into(),
            z: shape.iter().map(|b| b.z).min().unwrap() - 1.into(),
        };
        let max = Pos {
            x: shape.iter().map(|b| b.x).max().unwrap() + 1.into(),
            y: shape.iter().map(|b| b.y).max().unwrap() + 1.into(),
            z: shape.iter().map(|b| b.z).max().unwrap() + 1.into(),
        };
        Cube { min, max }
    }

    fn contains(&self, pos: &Pos<T>) -> bool {
        (self.min.x..=self.max.x).contains(&pos.x)
            && (self.min.y..=self.max.y).contains(&pos.y)
            && (self.min.z..=self.max.z).contains(&pos.z)
    }

    fn size_x(&self) -> usize {
        (self.max.x - self.min.x + 1.into()).try_into().unwrap_or(0)
    }

    fn size_y(&self) -> usize {
        (self.max.y - self.min.y + 1.into()).try_into().unwrap_or(0)
    }

    fn size_z(&self) -> usize {
        (self.max.z - self.min.z + 1.into()).try_into().unwrap_or(0)
    }

    fn num_exposed_faces(&self) -> usize {
        let sx = self.size_x();
        let sy = self.size_y();
        let sz = self.size_z();
        (2 * sx * sy) + (2 * sy * sz) + (2 * sx * sz)
    }
}

// flood fill from cube's min corner, without going out cube
fn flood_fill_in<T: PosValue>(shape: &mut HashSet<Pos<T>>, cube: &Cube<T>) -> usize {
    if shape.contains(&cube.min) {
        return 0;
    }
    let mut q = vec![cube.min];
    let mut filled = 1;
    while let Some(next) = q.pop() {
        if shape.contains(&next) {
            continue;
        }
        filled += 1;
        shape.insert(next);
        for nb in next.neighbours() {
            if cube.contains(&nb) && !shape.contains(&nb) {
                q.push(nb);
            }
        }
    }
    filled
}

fn exterior_surface_area<T: PosValue>(shape: &HashSet<Pos<T>>) -> usize {
    // flood fill outside, calculate surface are and subtract
    let cube = Cube::containing(shape);
    let total_exposed_faces = surface_area(shape);
    let mut filled_shape = shape.clone();
    flood_fill_in(&mut filled_shape, &cube);
    let filled_exposed_faces = surface_area(&filled_shape);
    let cube_faces = cube.num_exposed_faces();
    total_exposed_faces - (filled_exposed_faces - cube_faces)
}

fn main() {
    let input = read_input();

    println!("Surface: {}", surface_area(&input));
    println!("Exterior surface: {}", exterior_surface_area(&input));
}
