use std::io::{self, BufRead};

fn read_input() -> Result<Vec<u8>, std::io::Error> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    // read numbers
    reader.read_line(&mut buf)?;

    return Ok(buf.split(|c: char| !c.is_numeric() )
    .filter(|s| !s.is_empty() )
    .map(|s| u8::from_str_radix(s, 10).unwrap())
    .collect());
}

fn run_day(fish: &mut Vec<u8>) -> u32 {
    let mut births = 0;
    for f in fish {
        if *f == 0 {
            // new fish
            *f = 6;
            births += 1;
        } else {
            *f -= 1;
        }
    }
    return births;
}

fn print_fish(day: i32, fish: &Vec<u8>) {
    if day == 0 {
        print!("Initial state: ");
    } else {
        print!("After {:>2} days: ", day);
    }
    for f in fish {
        print!("{},", f);
    }
    print!("\n");
}

fn main() {
    let mut fish = read_input().ok().unwrap();
    print_fish(0, &fish);
    for day in 1..=80 {
        let births = run_day(&mut fish);
        for b in 0..births {
            fish.push(8);
        }
        //print_fish(day, &fish);
    }
    println!("total: {}", fish.len());
}
