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
        write!(f, "({},{})", self.x, self.y);
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
        if target.x.contains(&pos.x) {
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
    for vx in 1..=*target.x.end() {
        if simulate_x(&Coord::from(vx,0), target) {
            if min_vx == 0 { min_vx = vx; }
            max_vx = vx;
        }
    }

    RangeInclusive::new(min_vx, max_vx)
}

fn simulate_y(vy: &CoordType, target: &RangeInclusive<CoordType>) -> bool {
    let mut y = 0;
    let mut velocity = *vy;
    
    loop {
        y = y + velocity;
        if target.contains(&y) {
            return true;
        } else if y < *target.end() {
            return false;
        }

        // reduce velocity due to drag
        velocity -= 1;
    }
}

fn find_vy(target: &Area) -> RangeInclusive<CoordType> {
    *target.y.start()..=-target.y.start()-1
    /*
    let mut min_vy = 0;
    let mut max_vy = 0;
    for vy in *target.y.start()..=1000 {
        if simulate_y(&vy, &target.y) {
            if min_vy == 0 { min_vy = vy; }
            max_vy = vy;
        }
    }

    RangeInclusive::new(min_vy, max_vy)*/
}

fn main() {
    let target = Area::from(20..=30, -10..=-5);
    other_main(&target);
    let target = Area::from(195..=238, -93..=-67);
    other_main(&target);

}
fn other_main(target: &Area) {
    let vx_range = find_vx(&target);
    dbg!(&vx_range);
    let vy_range = find_vy(&target);
    dbg!(&vy_range);

    let mut valid_velocities = vec![];
    for vx in vx_range {
        for vy in *vy_range.start()..=*vy_range.end() {
            let v = Coord::from(vx, vy);
            let result = simulate(&v, &target);
            if result.1 {
                valid_velocities.push(v);
            }
        }
    }
    
    /*for (i,v) in valid_velocities.iter().enumerate() {
        println!("{}: {},{}", 1+i, v.x, v.y);
    }*/
    println!("{} results", valid_velocities.len());
}
