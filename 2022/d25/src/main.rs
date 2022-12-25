use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
    vec,
};

const NEGATIVE_BASE_DIGITS: [char; 4] = ['-', '=', '!', '#'];

fn decode_snafu(s: &str, base_range: &RangeInclusive<i8>) -> i64 {
    assert!(base_range.contains(&0), "Base must contain 0");
    let base = 1 + (base_range.end() - base_range.start()) as i64;
    let positive_radix = (1 + *base_range.end()) as u32;
    s.chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            let place = base.pow(idx as u32);
            place
                * match c {
                    x if NEGATIVE_BASE_DIGITS.contains(&x) => {
                        -(NEGATIVE_BASE_DIGITS.iter().position(|&p| p == x).unwrap() as i64 + 1)
                    }
                    x if x.is_digit(positive_radix) => x.to_digit(positive_radix).unwrap() as i64,
                    _ => panic!("Invalid digit {}", c),
                }
        })
        .sum()
}

fn encode_snafu(i: i64, base_range: &RangeInclusive<i8>) -> String {
    assert!(base_range.contains(&0), "Base must contain 0");
    let base = 1 + (base_range.end() - base_range.start());
    let max = *base_range.end();
    let mut r = i;
    let mut digits: Vec<i8> = vec![];
    // calculate digts
    let mut place_value = 1i64;
    for _ in 0i64.. {
        let next_place_value = place_value * (base as i64);
        let digit_value = r % next_place_value;
        let digit = digit_value / place_value;
        digits.push(digit as i8);
        r -= digit_value;
        if r == 0 {
            break;
        }
        place_value = next_place_value;
    }
    // adjust for carry
    digits.push(0);
    for i in 0..digits.len() - 1 {
        if digits[i] > max {
            digits[i + 1] += 1;
            digits[i] -= base;
        }
    }
    // TODO: if number is negative, adjustment for undercarry might be needed

    let positive_radix = 1 + *base_range.end();
    digits
        .iter()
        .rev()
        .skip_while(|&&d| d == 0)
        .map(|&d| match d {
            x if x >= 0 && x < positive_radix => {
                char::from_digit(d as u32, positive_radix as u32).unwrap()
            }
            x if x < 0 && x >= -(NEGATIVE_BASE_DIGITS.len() as i8) => {
                NEGATIVE_BASE_DIGITS[(-x - 1) as usize]
            }
            x => panic!("Don't know how to represent {}", x),
        })
        .map(|c| String::from(c))
        .collect()
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().filter_map(Result::ok).collect()
}

fn main() {
    let input = read_input();
    let base_range = -2..=2;

    let sum: i64 = input.iter().map(|ln| decode_snafu(&ln, &base_range)).sum();
    println!("Sum: {}", sum);
    let encoded = encode_snafu(sum, &base_range);
    println!("Encode: {}", encoded);
    assert_eq!(sum, decode_snafu(&encoded, &base_range));
}
