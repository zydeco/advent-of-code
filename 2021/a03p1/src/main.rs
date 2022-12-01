
use std::io::{self, BufRead};

fn read_stdin_bins() -> Vec<u32> {
    let stdin = io::stdin();
    let mut lines: Vec<u32> = Vec::new();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        lines.push(u32::from_str_radix(&s, 2).ok().unwrap());
    }
    return lines;
}

fn main() {
    let values = read_stdin_bins();
    let bits = 12;
    let mut counts =  vec![0; bits];

    let threshold = values.len() as u32 / 2;
    for val in values {
        for b in 0..bits {
            counts[b] += (val >> b) & 1;
        }
    }

    let mut gamma: u32 = 0;
    for (i,c) in counts.iter().enumerate() {
        let value = *c;
        println!("{}: {}", i, value);
        if value > threshold {
            gamma += 1 << i;
        }
    }

    println!("gamma: {}", gamma);
    let epsilon = !gamma & (1 << bits) - 1;
    println!("epsilon: {}", epsilon);
    println!("result: {}", epsilon * gamma);
}