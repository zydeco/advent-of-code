
use std::io::{self, BufRead};

fn read_input() -> (usize, Vec<u32>) {
    let stdin = io::stdin();
    let mut lines: Vec<u32> = Vec::new();
    let mut bits = 0;
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        bits = s.len();
        lines.push(u32::from_str_radix(&s, 2).ok().unwrap());
    }
    return (bits, lines);
}

fn count_ones(bits: usize, values: &Vec<u32>) -> Vec<usize> {
    let mut counts = vec![0 as usize; bits];
    for val in values {
        for b in 0..bits {
            if (val >> b) & 1 == 1 {
                counts[b] += 1;
            }
        }
    }
    return counts;
}

fn most_common_bits(bits: usize, values: &Vec<u32>) -> Vec<Option<bool>> {
    let counts = count_ones(bits, values);
    let total = values.len();
    return counts.into_iter().map(|ones| {
        let zeros = total - ones;
        if zeros == ones {
            None
        } else if zeros > ones {
            Some(false)
        } else {
            Some(true)
        }
    }).collect()
}

fn filter_values<F>(bits: usize, values: &Vec<u32>, filter: F) -> u32 where
    F: Fn(bool, Option<bool>) -> bool {

    let mut x = values.clone();
    for b in (0..bits).rev() {
        let commons = most_common_bits(bits, &x);
        x = x.into_iter().filter(|c| filter((*c >> b) & 1 == 1, commons[b])).collect::<Vec<_>>();
        
        
        if x.len() == 1 {
            return x[0]
        }

    }
    return 0
}

fn main() {
    let (bits, values) = read_input();

    let oxygen = filter_values(bits, &values, |bit,most_common| {
        return (most_common == None && bit == true) || most_common != None && bit == most_common.unwrap();
    });
    println!("oxygen {}", oxygen);
    let co2 = filter_values(bits, &values, |bit,most_common| {
        return (most_common == None && bit == false) || most_common != None && bit != most_common.unwrap();
    });
    println!("co2 {}", co2);
    println!("total {}", oxygen * co2);
}