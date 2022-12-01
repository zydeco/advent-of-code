use std::io::{self, BufRead};
use bitvec::prelude::*;

type EnhancementAlgorithm = BitArr!(for 512, in u8);

struct Image {
    lines: Vec<BitVec<Lsb0, u8>>,
    background: bool
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for line in self.lines.iter() {
            for bit in line.iter() {
                write!(f, "{}", if bit == true { '#' } else { '.' })?;
            }
            write!(f,"\n")?;
        }
        Ok(())
    }
}

impl Image {
    fn count_ones(&self) -> usize {
        self.lines.iter().fold(0, |acc, line| acc + line.count_ones())
    }

    fn enhance(&self, algo: &EnhancementAlgorithm) -> Image {
        let mut lines = vec![];
        let width = self.lines[0].len() as i32;
        let height = self.lines.len() as i32;
        let background = algo[if self.background { 511 } else { 0 }];

        for y in -2..height+2 {
            let mut line: BitVec<Lsb0, u8> = BitVec::repeat(false, width as usize + 4);
            for x in -2..width+2 {
                let pattern = self.pattern9(x, y);
                let pixel = algo[pattern as usize];
                line.as_mut_bitslice().set((x+2) as usize, pixel);
            }
            lines.push(line);
        }

        let mut image = Image{lines:lines, background: background};
        image.reduce();
        image
    }

    fn reduce(&mut self) {
        let check_line = if self.background {
            | line: &BitVec<Lsb0, u8> | {line.all()}
        } else {
            | line: &BitVec<Lsb0, u8> | {line.not_any()}
        };
        while check_line(&self.lines[0]) {
            self.lines.remove(0);
        }
        while check_line(&self.lines.last().unwrap()) {
            self.lines.remove(self.lines.len()-1);
        }

        let check_col = | col: usize | { self.lines.iter().all(|line| line[col] == self.background ) };
        let mut remove_front = 0;
        while check_col(remove_front) {
            remove_front += 1;
        }
        let mut remove_back = 0;
        let width = self.lines[0].len();
        let last_col = width - 1;
        while check_col(last_col - remove_back) {
            remove_back += 1;
        }

        for line in self.lines.iter_mut() {
            line.drain(width-remove_back..width);
            line.drain(0..remove_front);
        }
    }

    fn pixel(&self, x: i32, y: i32) -> bool {
        let width = self.lines[0].len() as i32;
        let height = self.lines.len() as i32;
        if x < 0 || y < 0 || x >= width || y >= height {
            return self.background;
        }
        return self.lines[y as usize][x as usize];
    }

    fn pattern9(&self, x: i32, y: i32) -> u16 {
        [
            self.pixel(x-1, y-1),
            self.pixel(x,   y-1),
            self.pixel(x+1, y-1),
            self.pixel(x-1, y),
            self.pixel(x,   y),
            self.pixel(x+1, y),
            self.pixel(x-1, y+1),
            self.pixel(x,   y+1),
            self.pixel(x+1, y+1),
        ].iter()
        .fold(0, |acc, next| (acc << 1) + if *next { 1 } else { 0 })
    }
}

fn read_input() -> std::io::Result<(EnhancementAlgorithm, Image)> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // read algorithm
    let mut line = String::new();
    let mut algo = BitArray::<Lsb0, [u8; 64]>::zeroed();
    reader.read_line(&mut line)?;
    let mb = algo.as_mut_bitslice();
    for (i,c) in line.chars().enumerate() {
        if c == '#' || c == '.' {
            mb.set(i, c == '#');
        }
    }

    // empty line
    line.clear();
    reader.read_line(&mut line)?;

    // image
    let mut image_lines = vec![];
    for line in reader.lines().filter(Result::is_ok).map(Result::ok).filter(Option::is_some).map(Option::unwrap).filter(|line| line.len() > 0) {
        let mut image_line: BitVec<Lsb0, u8> = BitVec::repeat(false, line.len());
        for (i,c) in line.chars().enumerate() {
            image_line.set(i, c == '#');
        }
        image_lines.push(image_line);
    }

    Ok((algo, Image{lines:image_lines, background: false}))
}

fn main() {
    let (algo, mut image) = read_input().unwrap();
    
    for _ in 0..50 {
        //println!("\x1B[2J\x1B[1;1HGeneration: {}\n{}", generation, image);
        image = image.enhance(&algo);
        //std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("ones: {}", image.count_ones());
    
}
