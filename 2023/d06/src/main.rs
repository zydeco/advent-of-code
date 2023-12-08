use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Race {
    time: i64,
    distance: i64,
}

// distance = speed * (time - time_pressed)
// speed = time_pressed
// time_pressed ∈ [0, time]

fn concat(a: i64, b: i64) -> i64 {
    a * (10i64.pow(1 + b.ilog10())) + b
}

impl Race {
    fn distance(&self, time_pressed: i64) -> i64 {
        if time_pressed >= self.time {
            return 0;
        }
        return time_pressed * (self.time - time_pressed);
    }

    fn winning_moves(&self) -> usize {
        let range = 1..self.time;
        range
            .map(|t| self.distance(t) > self.distance)
            .filter(|&is_better| is_better)
            .count()
    }

    fn concat(&self, other: &Race) -> Race {
        Race {
            time: concat(self.time, other.time),
            distance: concat(self.distance, other.distance),
        }
    }
}

fn read_numbers(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .filter_map(|word| i64::from_str_radix(word, 10).ok())
        .collect()
}

fn read_input() -> Vec<Race> {
    let mut lines = io::stdin().lock().lines().filter_map(Result::ok);
    let time = read_numbers(&lines.next().unwrap()[10..]);
    let distance = read_numbers(&lines.next().unwrap()[10..]);
    time.iter()
        .zip(distance)
        .map(|(&time, distance)| Race { time, distance })
        .collect()
}

fn main() {
    let races = read_input();
    let ways = races.iter().map(Race::winning_moves).fold(1, |a, b| a * b);
    println!("Part 1: {}", ways);

    // part 2
    let race = races.iter().skip(1).fold(races[0], |r, s| r.concat(s));
    println!("Final race: {:?}", race);
    println!("Part 2: {}", race.winning_moves());
}
