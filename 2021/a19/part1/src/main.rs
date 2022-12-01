mod coords;

use std::io::{self, BufRead};
use std::collections::{HashSet,HashMap};
use crate::coords::{Coord,Facing,Rotation,FLIPS,inverse_flip};

fn read_input() -> Vec<Vec<Coord>> {
    let mut scanners = vec![];
    let mut coords = vec![];
    for line in io::stdin().lock().lines().filter(Result::is_ok).map(Result::unwrap) {
        if line.len() == 0 && coords.len() > 0 {
            // new scanner
            scanners.push(coords);
            coords = vec![];
        } else if line.starts_with("---") {
            // comment
        } else if let Some(coord) = Coord::from_str(&line) {
            // coordinate
            coords.push(coord);
        }
    }
    if coords.len() > 0 {
        scanners.push(coords);
    }
    scanners
}

fn overlaps(beacons0: &Vec<Coord>, beacons1: &Vec<Coord>) -> Option<(Vec<Coord>, Facing, Rotation, Coord)> {
    for c0 in beacons0 {
        let beacons0_relative = beacons0.iter().map(|c| c - c0 ).collect::<HashSet<_>>();
        for (facing, rotation) in FLIPS {
            let flipped = beacons1.iter().map(|coord| coord.rotate(facing, rotation)).collect::<Vec<_>>();
            for c1 in flipped.iter() {
                let beacons1_relative = flipped.iter().map(|c| c - c1 ).collect::<HashSet<_>>();
                let intersection = beacons0_relative.intersection(&beacons1_relative).map(|x| *x).collect::<Vec<_>>();
                if intersection.len() >= 12 {
                    // intersection is in relative coordinates to c0
                    // convert back to original coordinates
                    let intersection_from_b0 = intersection.iter().map(|c| c + c0).collect::<Vec<_>>();
                    // relative coordinates of scanner 1 in scanner 0 space
                    let rel = c0 - c1;
                    return Some((intersection_from_b0, facing, rotation, rel));
                }
            }
        }
    }
    None
}

type RelMap = HashMap<(usize,usize), (Coord,Facing,Rotation)>;

fn convert_coordinate(coord: &Coord, from: usize, to: usize, mapping: &RelMap) -> Option<Coord> {
    convert_coordinate_sub(coord, from, to, mapping, 0)
}

fn convert_coordinate_sub(coord: &Coord, from: usize, to: usize, mapping: &RelMap, steps: u128) -> Option<Coord> {
    if from == to {
        return Some(*coord)
    } else if let Some((base, facing, rotation)) = mapping.get(&(from, to)) {
        return Some(coord.rotate(*facing, *rotation) + *base);
    }
    for (_,step) in mapping.keys().filter(|(f,t)| *f == from && steps & (1u128 << t) == 0 ) {
        let new_steps = steps | (1u128 << step);
        let intermediate = convert_coordinate_sub(coord, from, *step, mapping, new_steps).unwrap();
        let end_coord = convert_coordinate_sub(&intermediate, *step, to, mapping, new_steps);
        if end_coord.is_some() {
            return end_coord;
        }
    }
    return None;
}

fn main() {
    let input = read_input();
    let num_scanners = input.len();
    let mut rel_map: RelMap = HashMap::new();

    for i in 0..num_scanners {
        for j in i+1..num_scanners {
            if let Some((coords, facing, rotation, base)) = overlaps(&input[i], &input[j]) {
                println!("Scanner {} overlaps with {}: relative {}{}{} ({} beacons)", i, j, base, facing, rotation, coords.len());
                rel_map.insert((j,i), (base, facing, rotation));
                // inverse mapping
                let (inv_facing, inv_rotation) = inverse_flip(facing, rotation);
                let inv_base = (-base).rotate(inv_facing, inv_rotation);
                rel_map.insert((i,j), (inv_base, inv_facing, inv_rotation));

                /*println!("  Relative to {} -> {} -> 0:", i, j);
                for c in &coords {
                    let c1 = convert_coordinate(c, i, j, &rel_map).unwrap();
                    let c0 = convert_coordinate(c, i, 0, &rel_map).unwrap();
                    println!("    {} -> {} -> {}", c, c1, c0);
                }*/
            }
        }
    }

    // part 1
    let mut beacons: HashSet<Coord> = HashSet::new();
    for (i,coords) in input.iter().enumerate() {
         for c in coords {
            if let Some(c0) = convert_coordinate(c, i, num_scanners-1, &rel_map) {
                beacons.insert(c0);
            } else {
                panic!("Couldn't convert from {} to {}", i, num_scanners-1);
            }
         }
    }
    println!("beacons: {}", beacons.len());

    // part 2
    let scanner_positions = (0..num_scanners).map(|i| convert_coordinate(&Coord::default(), i, 0, &rel_map).unwrap()).collect::<Vec<_>>();
    let mut max_distance = 0;
    for i in scanner_positions.iter() {
        for j in scanner_positions.iter() {
            let distance = i.manhattan_distance(j);
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    println!("Max distance {}", max_distance);
}
