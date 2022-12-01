use std::io::{self, BufRead, Read};

fn read_input() -> Vec<Vec<u32>> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    buf.split("\n\n")
        .map(|elf_str| {
            elf_str
                .split('\n')
                .filter(|ln| !ln.is_empty())
                .map(|ln| ln.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let elves = read_input();
    let mut sums = elves
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<Vec<_>>();
    sums.sort();

    let max = sums.last();
    println!("max: {}", max.unwrap());

    let max3 = sums.iter().rev().take(3).sum::<u32>();
    println!("max 3: {}", max3);
}
