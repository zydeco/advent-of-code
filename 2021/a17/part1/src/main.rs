use std::ops::{RangeInclusive, Add, Sub};
use std::fmt::Debug;

type CoordType = i32;

#[derive(Debug, Copy, Clone, Default)]
struct Coord {
    x: CoordType,
    y: CoordType
}

#[derive(Debug)]
struct Area {
    x: RangeInclusive<CoordType>,
    y: RangeInclusive<CoordType>
}

impl Coord {
    fn from(x: CoordType, y: CoordType) -> Coord {
        Coord{x: x, y: y}
    }

    fn add(self, other: (CoordType,CoordType)) -> Self {
        Self {x: self.x + other.0, y: self.y + other.1}
    }

    fn sub(self, other: (CoordType,CoordType)) -> Self {
        Self {x: self.x - other.0, y: self.y - other.1}
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)?;
        Ok(())
    }
}

impl Area {
    fn from(x: RangeInclusive<CoordType>, y: RangeInclusive<CoordType>) -> Area {
        Area{x: x, y:y}
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.x.contains(&coord.x) && self.y.contains(&coord.y)
    }

    fn overshot(&self, coord: &Coord) -> bool {
        coord.x > *self.x.end() || coord.y < *self.y.start()
    }
}

fn simulate(velocity: &Coord, target: &Area) -> (CoordType, bool) {
    let mut pos = Coord::from(0, 0);
    let mut velocity = *velocity;
    let mut max_y = 0;

    loop {
        pos = pos + velocity;
        if pos.y > max_y { max_y = pos.y; }
        if target.overshot(&pos) {
            return (0, false);
        } else if target.contains(&pos) {
            return (max_y, true);
        }

        // reduce velocity due to drag
        velocity.y -= 1;
        if velocity.x > 0 {
            velocity.x -= 1;
        } else if velocity.x < 0 {
            velocity.x += 1;
        }
    }
}

fn simulate_x(velocity: &Coord, target: &Area) -> bool {
    let mut pos = Coord::from(0, *target.y.start());
    let mut velocity = *velocity;
    
    loop {
        pos = pos + velocity;
        if target.overshot(&pos) {
            return false;
        } else if target.contains(&pos) {
            return true;
        }

        // reduce velocity due to drag
        if velocity.x > 0 {
            velocity.x -= 1;
        } else if velocity.x < 0 {
            velocity.x += 1;
        } else {
            // stopped
            return false;
        }
    }
}

fn find_vx(target: &Area) -> RangeInclusive<CoordType> {
    let mut min_vx = 0;
    let mut max_vx = 0;
    for vx in 1..*target.x.end() {
        if simulate_x(&Coord::from(vx,0), target) {
            if min_vx == 0 { min_vx = vx; }
            max_vx = vx;
        }
    }

    RangeInclusive::new(min_vx, max_vx)
}

fn main() {
    //let target = Area::from(20..=30, -10..=-5);
    let target = Area::from(195..=238, -93..=-67);
    
    let vx_range = find_vx(&target);
    dbg!(&vx_range);
    let mut max_y = 0;
    let vx = *vx_range.start();
    for vy in 1..=1000 {
        let result = simulate(&Coord::from(vx, vy), &target);
        if result.1 && result.0 > max_y {
            max_y = result.0;
        }
    }
    
    println!("max_y {}", max_y);
}
