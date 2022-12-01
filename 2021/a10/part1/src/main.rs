use std::io::{self, BufRead};

fn is_opener(c: &char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false
    }
}

fn is_closer(c: &char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false
    }
}

fn matching_closer(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("not an opener!")
    }
}

fn bad_closer_score(c: &char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn completion_score(c: &char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }
}

fn read_line(buf: &String) -> Vec<char> {
    buf
        .chars()
        .filter(|c| is_opener(c) || is_closer(c) )
        .collect::<Vec<_>>()
}

fn read_input() -> Vec<Vec<char>> {
    io::stdin().lock().lines()
        .map(|line| read_line(&line.ok().unwrap()) )
        .collect()
}

fn main() {
    let input = read_input();

    let mut part1_score = 0u32;
    let mut completion_scores = vec![];
    'line: for line in input {
        let mut closers = vec![];
        for c in line {
            if is_opener(&c) {
                closers.push(matching_closer(&c));
            } else if c == closers.pop().unwrap() {
                // valid closer

            } else {
                // invalid closer
                part1_score += bad_closer_score(&c);
                continue 'line;
            }
        }
        // completion score
        let mut line_comp_score = 0u64;
        for c in closers.into_iter().rev() {
            line_comp_score = 5 * line_comp_score + completion_score(&c) as u64;
        }
        completion_scores.push(line_comp_score);
    }
    println!("part 1 score {}", part1_score);

    completion_scores.sort();
    println!("part 2 score {}", completion_scores[completion_scores.len() / 2]);

}
