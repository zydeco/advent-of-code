use std::{
    env,
    io::{self, BufRead},
};

fn decode_snafu(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            let place = 5i64.pow(idx as u32);
            place
                * match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!("Invalid digit {}", c),
                }
        })
        .sum()
}

fn encode_snafu(i: i64) -> String {
    let mut r = i;
    let mut place = 5i64.pow((i as f64).log(5f64).ceil() as u32);
    let mut digits: Vec<char> = vec![];
    while place > 0 {
        let mut this_digit = r / place;
        let next_place = place / 5;
        println!(
            "r={}, place={}[d{}], digit={}",
            r,
            place,
            (place as f64).log(5f64).floor() as u32,
            this_digit
        );
        if place > 1 {
            let next_r = r - (this_digit * place);
            println!(" next_r / next_place = {}", next_r / next_place);
            if next_r / next_place > 2 {
                println!("  this_digit +1");
                this_digit += 1;
            } else if next_r / next_place < -2 {
                println!("  this_digit -1");
                this_digit -= 1;
            }
        }

        r -= this_digit * place;
        place = next_place;

        assert!(
            this_digit >= -2 && this_digit <= 2,
            "digit was {}",
            this_digit
        );

        digits.push(match this_digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        });
    }

    digits.iter().skip_while(|&d| d.eq(&'0')).collect()
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().filter_map(Result::ok).collect()
}

fn main() {
    let input = read_input();

    let sum: i64 = input.iter().map(|ln| decode_snafu(&ln)).sum();
    println!("Sum: {}", sum);

    let arg = env::args().skip(1).next().unwrap().parse::<i64>().unwrap();
    let encoded = encode_snafu(arg);
    println!("Encode {}: {}", arg, encoded);
    assert_eq!(arg, decode_snafu(&encoded));
}
