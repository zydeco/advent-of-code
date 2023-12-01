use std::io::{self, BufRead};
use regex::{Regex, Captures};

fn get_number(s: String) -> u32 {
    let first = s.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = s.chars().filter_map(|c| c.to_digit(10)).last().unwrap();
    return 10*first+last;
}

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn digit_from(s: &str) -> u32 {
    if s.len() == 1 {
        return s.chars().next().unwrap().to_digit(10).unwrap()
    }
    
    let pos = NUMBERS.iter().position(|&r| r == s).unwrap() as u32;
    return pos+1
}

fn get_number_b(s: String) -> u32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();
    let first = re.find(&s).unwrap().as_str();
    let last = (1..=s.len()).find_map(|i| re.find_at(&s, s.len() - i)).unwrap().as_str();
    return 10 * digit_from(first) + digit_from(last)
}

fn main() {
    let val = io::stdin()
    .lock()
    .lines()
    .filter_map(Result::ok)
    .map(get_number_b)
    .fold(0, |s, x| s + x);

    println!("The value is {}!", val);
}
