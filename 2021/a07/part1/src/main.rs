use std::io::{self, BufRead};

fn read_input() -> Result<Vec<i32>, std::io::Error> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    // read numbers
    reader.read_line(&mut buf)?;

    return Ok(buf.split(|c: char| !c.is_numeric() )
    .filter(|s| !s.is_empty() )
    .map(|s| i32::from_str_radix(s, 10).unwrap())
    .collect());
}

fn fuel_to<F>(target: i32, input: &Vec<i32>, cost: F) -> i32
where F: Fn(i32) -> i32
{
    input.into_iter().fold(0, |sum, x| sum + cost((target - x).abs()))
}

fn main() {
    let mut input = read_input().ok().unwrap();
    input.sort();
    
    // part 1: linear cost
    let target = input[input.len() / 2];
    println!("[part 1] Fuel to {}: {}", target, fuel_to(target, &input, |x| x));

    // part 2: exponential? cost
    let last = *input.last().unwrap();
    let mut cost = i32::MAX;
    let mut cheapest_target = 0;
    for target in input[0]..=last {
        let cur = fuel_to(target, &input, |dist| (dist * (dist + 1)) / 2);
        if cur < cost {
            cost = cur;
            cheapest_target = target;
        }

    }
    println!("[part 2] Fuel to {}: {}", cheapest_target, cost);
}
