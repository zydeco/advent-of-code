use std::{
    collections::HashSet,
    io::{self, BufRead},
    ops::RangeInclusive,
    str::FromStr,
};

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SensorReading {
    sensor: Pos,
    closest_beacon: Pos,
}

impl FromStr for SensorReading {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(|c: char| c != '-' && !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        assert!(values.len() == 4);
        Ok(SensorReading {
            sensor: (values[0], values[1]),
            closest_beacon: (values[2], values[3]),
        })
    }
}

fn read_input() -> Vec<SensorReading> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|ln| SensorReading::from_str(&ln).ok())
        .collect()
}

impl SensorReading {
    fn range(&self) -> i32 {
        (self.sensor.0 - self.closest_beacon.0).abs()
            + (self.sensor.1 - self.closest_beacon.1).abs()
    }

    fn range_at_row(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let range = self.range();
        let offset = (y - self.sensor.1).abs();
        if offset > range {
            None
        } else {
            let x1 = self.sensor.0 - range + offset;
            let x2 = self.sensor.0 + range - offset;
            Some(x1..=x2)
        }
    }
}

fn count_x_where_no_beacon(readings: &Vec<SensorReading>, y: i32) -> usize {
    let mut ranges = readings
        .iter()
        .filter_map(|sr| sr.range_at_row(y))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| *r.start());

    let beacons_on_line = readings
        .iter()
        .filter(|sr| sr.closest_beacon.1 == y)
        .map(|sr| sr.closest_beacon.0)
        .collect::<HashSet<_>>()
        .len();
    let start = *ranges[0].start();
    let end = *ranges.last().unwrap().end();
    (start..=end)
        .filter(|x| ranges.iter().any(|r| r.contains(x)))
        .count()
        - beacons_on_line
}

fn find_empty_spot(mut rs: &[RangeInclusive<i32>], max: i32) -> Option<i32> {
    // skip before zero
    while !rs.is_empty() && rs[0].end().lt(&0) {
        rs = &rs[1..];
    }
    if rs.is_empty() {
        return None;
    }

    let mut candidate = 0;
    while !rs.is_empty() {
        let (start, end) = (*rs[0].start(), *rs[0].end());
        if start > candidate {
            return Some(candidate);
        } else if end >= max {
            return None;
        }
        if end > candidate {
            candidate = end + 1;
        }
        rs = &rs[1..];
    }
    None
}

fn find_distress_signal(readings: &Vec<SensorReading>, max: i32) -> Option<Pos> {
    for y in 0..=max {
        let mut ranges = readings
            .iter()
            .filter_map(|sr| sr.range_at_row(y))
            .collect::<Vec<_>>();
        ranges.sort_by_key(|r| *r.start());
        if let Some(x) = find_empty_spot(ranges.as_slice(), max) {
            return Some((x, y));
        }
    }

    None
}

fn print_field(readings: &Vec<SensorReading>, range: RangeInclusive<Pos>) {
    let x_range = range.start().0..=range.end().0;
    let y_range = range.start().1..=range.end().1;
    let sensors = readings.iter().map(|sr| sr.sensor).collect::<HashSet<_>>();
    let beacons = readings
        .iter()
        .map(|sr| sr.closest_beacon)
        .collect::<HashSet<_>>();
    for y in y_range {
        let ranges = readings
            .iter()
            .filter_map(|sr| sr.range_at_row(y))
            .collect::<Vec<_>>();
        print!("{:<4}", y);
        for x in x_range.clone() {
            let pos = (x, y);
            print!(
                "{}",
                if sensors.contains(&pos) {
                    'S'
                } else if beacons.contains(&pos) {
                    'B'
                } else if ranges.iter().any(|r| r.contains(&x)) {
                    '#'
                } else {
                    '.'
                }
            )
        }
        print!("\n");
    }
}

fn pos_value(pos: Pos) -> u64 {
    (pos.0 as u64 * 4000000u64) + pos.1 as u64
}

fn main() {
    let input = read_input();
    let max = input.iter().map(|sr| sr.sensor.1).max().unwrap();
    let big = max > 20;

    // part 1
    let line = if big { 2000000 } else { 10 };
    println!("Part 1: {}", count_x_where_no_beacon(&input, line));

    if !big {
        print_field(&input, (0, 0)..=(20, 20));
    }

    // part 2
    let max_x = if big { 4000000 } else { 20 };
    let ds = find_distress_signal(&input, max_x).unwrap();
    println!("Part 2: {}", pos_value(ds));
}
