mod snailfish_math;
use crate::snailfish_math::Number;
use std::io::{self, BufRead};

fn read_input() -> Vec<Number> {
    let stdin = io::stdin();
    let mut numbers: Vec<Number> = vec![];
    for line in stdin.lock().lines() {
        if let Some(x) = Number::from_str(&line.unwrap()) {
            numbers.push(x);
        }
    }
    return numbers;
}

fn main() {
    let input = read_input();

    let sum = input.iter().fold(Number::default(), |acc,n| acc + n );
    println!("Magnitude: {}", sum.magnitude());
    
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let sum = &input[i] + &input[j];
            let mag = sum.magnitude();
            if mag > max {
                max = mag;
            }
            let sum = &input[j] + &input[i];
            let mag = sum.magnitude();
            if mag > max {
                max = mag;
            }
        }
    }
    println!("Max magintude: {}", max);
}
