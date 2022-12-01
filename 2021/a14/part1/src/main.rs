use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    input: [char; 2],
    output: char
}

impl Rule {
    fn from_str(input: &str) -> Rule {
        // "VF -> S"
        let chars: Vec<char> = input.chars().filter(char::is_ascii_uppercase).collect();
        Rule {
            input: [chars[0], chars[1]],
            output: chars[2]
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}{} -> {}", self.input[0], self.input[1], self.output);
        Ok(())
    }
}

fn read_input() -> Result<(Vec<char>, Vec<Rule>), std::io::Error> {
    let stdin = io::stdin();
    let mut rules: Vec<Rule> = vec![];
    
    let mut buf = String::new();
    let mut reader = stdin.lock();
    // read template
    reader.read_line(&mut buf)?;
    let template = buf.chars().filter(char::is_ascii_uppercase).collect::<Vec<_>>();

    // read rules
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() >= 7 {
            rules.push(Rule::from_str(&line));
        }
    }
    Ok((template, rules))
}

fn apply_rules(rules: &Vec<Rule>, input: [char; 2]) -> Vec<char> {
    for rule in rules {
        if rule.input == input {
            return vec![input[0], rule.output];
        }
    }
    vec![input[0]]
}

fn expand(input: &Vec<char>, rules: &Vec<Rule>) -> Vec<char> {
    let input_len = input.len();
    let mut output = Vec::with_capacity(input_len * 2);
    for i in 0..input_len-1 {
        let mut result = apply_rules(rules, [input[i], input[i+1]]);
        output.append(&mut result);
    }

    output.push(input[input_len-1]);
    output
}

fn count_stuff(input: &Vec<char>) -> HashMap<char, usize> {
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in input {
        *count.entry(*c).or_default() += 1;
    }
    count
}

fn main() {
    let input = read_input().unwrap();
    let rules = input.1;

    let mut polymer = input.0;
    for i in 1..=10 {
        polymer = expand(&polymer, &rules);
        println!("Step {}:", i);
        count_stuff(&polymer);
    }
    let counts = count_stuff(&polymer);
    dbg!(&counts);

    let mut values = counts.values().collect::<Vec<_>>();
    values.sort();
    let total = values[values.len()-1] - values[0];
    println!("Total: {}", total);
}
