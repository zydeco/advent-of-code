use std::io::{self, BufRead};

fn read_input() -> Result<Vec<u8>, std::io::Error> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    // read numbers
    reader.read_line(&mut buf)?;

    return Ok(buf.split(|c: char| !c.is_numeric() )
    .filter(|s| !s.is_empty() )
    .map(|s| u8::from_str_radix(s, 10).unwrap())
    .collect());
}

fn run_day(ages: &mut [u64; 9]) -> [u64; 9] {
    return [
        ages[1],
        ages[2],
        ages[3],
        ages[4],
        ages[5],
        ages[6],
        ages[7] + ages[0],
        ages[8],
        ages[0]];
}

fn main() {
    let input = read_input().ok().unwrap();
    let mut ages = [0u64; 9];
    for f in input {
        ages[f as usize] += 1;
    }
    for _ in 1..=256 {
        ages = run_day(&mut ages);
    }
    let mut total = 0;
    for i in 0..=8 {
        total += ages[i];
    }
    println!("total: {}", total);
}
