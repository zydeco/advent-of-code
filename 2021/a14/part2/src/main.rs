use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_input() -> Result<(Vec<char>, HashMap<[char;2], char>), std::io::Error> {
    let stdin = io::stdin();
    let mut rules = HashMap::new();
    
    let mut buf = String::new();
    let mut reader = stdin.lock();
    // read template
    reader.read_line(&mut buf)?;
    let template = buf.chars().filter(char::is_ascii_uppercase).collect::<Vec<_>>();

    // read rules
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() >= 7 {
            let chars: Vec<char> = line.chars().filter(char::is_ascii_uppercase).collect();
            rules.insert([chars[0], chars[1]], chars[2]);
        }
    }
    Ok((template, rules))
}

fn apply_rules(rules: &HashMap<[char;2], char>, pairs: &mut HashMap<[char;2], u64>) {
    for (pair, count) in pairs.clone() {
        match rules.get(&pair) {
            Some(c) => {
                // count new pairs
                *pairs.entry([pair[0], *c]).or_default() += count;
                *pairs.entry([*c, pair[1]]).or_default() += count;
                // discount old pair
                *pairs.entry(pair).or_default() -= count;
            }
            None => ()
        }
    }
}

fn count_expand(input: &Vec<char>, rules: &HashMap<[char;2], char>, times: u8) -> HashMap<char, u64> {
    let mut pairs = HashMap::new();
    for i in 0..input.len()-1 {
        let pair = [input[i], input[i+1]];
        *pairs.entry(pair).or_default() += 1;
    }
    let mut counter = HashMap::new();
    *counter.entry(*input.last().unwrap()).or_default() += 1;
    for _ in 1..=times {
        apply_rules(rules, &mut pairs);
    }
    for (pair, count) in pairs {
        *counter.entry(pair[0]).or_default() += count;
    }
    counter
}

fn main() {
    let input = read_input().unwrap();
    let results = count_expand(&input.0, &input.1, 40);
    dbg!(&results);

    let mut counts = results.values().collect::<Vec<_>>();
    counts.sort();
    let total = counts[counts.len()-1] - counts[0];
    println!("Total: {}", total);
}
