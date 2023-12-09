use std::io::{self, BufRead};

fn parse_line(s: String) -> Vec<i32> {
    s.split_ascii_whitespace()
        .filter_map(|word| i32::from_str_radix(word, 10).ok())
        .collect()
}

fn read_input() -> Vec<Vec<i32>> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect()
}

fn diffs(values: &Vec<i32>) -> Vec<i32> {
    values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn next(values: &Vec<i32>) -> i32 {
    let diffs = diffs(values);
    let last = *values.last().unwrap();
    if diffs.iter().all(|&i| i == 0) {
        return last;
    }
    return last + next(&diffs);
}

fn prev(values: &Vec<i32>) -> i32 {
    let diffs = diffs(values);
    let first = values[0];
    if diffs.iter().all(|&i| i == 0) {
        return first;
    }
    return first - prev(&diffs);
}

fn main() {
    let input = read_input();
    let sums = input
        .iter()
        .map(|v| (next(v), prev(v)))
        .fold((0, 0), |(sum1, sum2), (a, b)| (sum1 + a, sum2 + b));
    println!("{:?}", sums);
}
