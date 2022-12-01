use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_word(word: &str) -> u8 {
    let mut val = 0u8;
    for b in word.chars() {
        val |= match b {
            'a' => (1 << 0),
            'b' => (1 << 1),
            'c' => (1 << 2),
            'd' => (1 << 3),
            'e' => (1 << 4),
            'f' => (1 << 5),
            'g' => (1 << 6),
            _ => panic!("unknown segment")
        }
    }
    val
}

fn read_line(buf: &String) -> [u8; 14]{
    buf
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty() )
        .map(|w| read_word(&w) )
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn read_input() -> Vec<[u8; 14]> {
    io::stdin().lock().lines()
        .map(|line| read_line(&line.ok().unwrap()) )
        .collect()
}

fn count_output_1478(input: &Vec<[u8; 14]>) -> usize {
    let mut count = 0usize;
    for i in input {
        let output = &i[10..14];
        for b in output {
            count += match b.count_ones() {
                2 => 1, // digit 1
                3 => 1, // digit 7
                4 => 1, // digit 4
                7 => 1, // digit 8
                _ => 0
            }
        }
    }
    count
}

fn guess_digit_encoding(data: &[u8; 14]) -> [u8; 10] {
    let mut encoding = [0u8; 10];
    let mut digits = *data;
    digits.sort_by_key(|d| d.count_ones());
    for d in digits {
        match d.count_ones() {
            2 => encoding[1] = d,
            3 => encoding[7] = d,
            4 => encoding[4] = d,
            5 => { // 2, 3 or 5
                if d & encoding[1] == encoding[1] && encoding[1] != 0 {
                    // 3
                    encoding[3] = d;
                } else if (d & encoding[4]).count_ones() == 2 && encoding[4] != 0 {
                    // 2
                    encoding[2] = d;
                } else if (d & encoding[4]).count_ones() == 3 && encoding[4] != 0 {
                    // 5
                    encoding[5] = d;
                } else {
                    panic!("you must construct additional pylons");
                }
            }
            6 => { // 0, 6 or 9
                if (encoding[1] != 0 && d & encoding[1] == encoding[1]) ||
                    (encoding[7] != 0 && d & encoding[7] == encoding[7])
                  {
                    // 0 or 9
                    if encoding[4] == 0 { panic!("need a 4") }
                    if (d & encoding[4]).count_ones() == 4 {
                        // 9
                        encoding[9] = d;
                    } else {
                        encoding[0] = d; 
                    }
                } else {
                    // 6
                    encoding[6] = d;
                }
            }
            7 => encoding[8] = d,
            _ => ()
        }
    }
    encoding
}


fn get_output(data: &[u8; 14]) -> u32 {
    let encoding = guess_digit_encoding(data);
    let encoding_map = HashMap::from([
        (encoding[0], 0u32),
        (encoding[1], 1u32),
        (encoding[2], 2u32),
        (encoding[3], 3u32),
        (encoding[4], 4u32),
        (encoding[5], 5u32),
        (encoding[6], 6u32),
        (encoding[7], 7u32),
        (encoding[8], 8u32),
        (encoding[9], 9u32)
    ]);
    let mut result = 0u32;
    let output_digits = &data[10..14];
    for d in output_digits {
        result = result * 10 + encoding_map[&d];
    }
    result
}

fn main() {
    let input = read_input();
    println!("outputs 1, 4, 7 and 8: {}", count_output_1478(&input));

    let mut result = 0;
    for sample in input {
        let value = get_output(&sample);
        //println!("{}", value);
        result += value;
    }
    println!("output total: {}", result);
}
