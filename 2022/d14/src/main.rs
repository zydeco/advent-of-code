use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, BufRead},
    ops::RangeInclusive,
    thread::sleep,
    time::Duration,
};

type Pos = (i32, i32);

const SOURCE: Pos = (500, 0);

fn read_pos(s: &str) -> Pos {
    let values = s
        .split(",")
        .map(|word| word.parse().unwrap())
        .take(2)
        .collect::<Vec<_>>();
    (values[0], values[1])
}

fn read_line(s: &str) -> Vec<Pos> {
    s.split(" -> ").map(read_pos).collect()
}

fn read_input() -> Vec<Vec<Pos>> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| read_line(&line))
        .collect()
}

fn pos_range(values: &mut dyn Iterator<Item = &Pos>) -> RangeInclusive<Pos> {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for pos in values {
        if pos.0 < min.0 {
            min.0 = pos.0;
        }
        if pos.0 > max.0 {
            max.0 = pos.0;
        }
        if pos.1 < min.1 {
            min.1 = pos.1;
        }
        if pos.1 > max.1 {
            max.1 = pos.1;
        }
    }
    min..=max
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Wall,
    Sand { falling: bool },
}

#[derive(Debug)]
struct Cave {
    range: RangeInclusive<Pos>,
    blocks: HashMap<Pos, Block>,
    floor: Option<i32>,
}

fn irange(x: i32, y: i32) -> RangeInclusive<i32> {
    if x < y {
        x..=y
    } else {
        y..=x
    }
}

impl Block {
    fn is_fixed(&self) -> bool {
        match self {
            Self::Sand { falling: true } => false,
            _ => true,
        }
    }
}

impl Cave {
    fn from(lines: &Vec<Vec<Pos>>, has_floor: bool) -> Self {
        let range = pos_range(&mut lines.iter().flat_map(|f| f.iter()));
        let mut blocks = HashMap::new();

        fn add_wall(blocks: &mut HashMap<Pos, Block>, from: &Pos, to: &Pos) {
            if from.0 == to.0 {
                for y in irange(from.1, to.1) {
                    blocks.insert((from.0, y), Block::Wall);
                }
            } else if from.1 == to.1 {
                for x in irange(from.0, to.0) {
                    blocks.insert((x, from.1), Block::Wall);
                }
            } else {
                panic!("Can't do diagonals")
            }
        }

        for line in lines {
            for i in 0..line.len() - 1 {
                add_wall(&mut blocks, &line[i], &line[i + 1]);
            }
        }

        let floor = if has_floor {
            Some(range.end().1 + 2)
        } else {
            None
        };

        Cave {
            range,
            blocks,
            floor,
        }
    }

    fn is_still(&self) -> bool {
        self.blocks.values().all(Block::is_fixed)
    }

    fn height(&self) -> usize {
        (self.range.end().1 - self.range.start().1) as usize
    }

    fn width(&self) -> usize {
        (self.range.end().0 - self.range.start().0) as usize
    }

    fn spawn(&mut self, pos: Pos, block: Block) -> bool {
        if self.has_block(pos) {
            return false;
        }
        self.blocks.insert(pos, block);
        true
    }

    fn has_block(&self, pos: Pos) -> bool {
        self.blocks.contains_key(&pos) || self.floor == Some(pos.1)
    }

    fn is_empty(&self, pos: Pos) -> bool {
        !self.has_block(pos)
    }

    // return true if something fell off the world
    fn tick(&mut self) -> bool {
        if self.is_still() {
            return false;
        }

        let falling_sands = self
            .blocks
            .iter()
            .filter(|(_, block)| Block::Sand { falling: true }.eq(block))
            .map(|(&pos, _)| pos)
            .collect::<Vec<_>>();

        assert!(falling_sands.len() == 1);
        let pos = falling_sands[0];
        let candidates = [
            (pos.0, pos.1 + 1),     // down
            (pos.0 - 1, pos.1 + 1), // down left
            (pos.0 + 1, pos.1 + 1), // down right
            (pos.0, pos.1),         // do not fall
        ];
        self.blocks.remove(&pos);
        if self.floor == None && pos.1 == self.range.end().1 {
            return true;
        }
        for new_pos in candidates {
            if self.is_empty(new_pos) {
                self.blocks.insert(
                    new_pos,
                    Block::Sand {
                        falling: new_pos.1 > pos.1,
                    },
                );
                break;
            }
        }

        false
    }
}

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = &self.range;
        let underscan = 2;
        for y in 0..=range.end().1 + underscan {
            for x in range.start().0 - underscan..=range.end().0 + underscan {
                write!(
                    f,
                    "{}",
                    match self.blocks.get(&(x, y)) {
                        Some(Block::Wall) => "🪨",
                        Some(Block::Sand { falling: _ }) => "🥪",
                        None if (x, y) == SOURCE => "🕳️",
                        None if Some(y) == self.floor => "🪨",
                        None => "  ",
                    }
                )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn can_fit_term(cols: usize, lines: usize) -> bool {
    if let Some((w, h)) = term_size::dimensions() {
        cols <= w && lines <= h
    } else {
        false
    }
}

fn simulate(lines: &Vec<Vec<Pos>>, has_floor: bool) -> usize {
    let mut cave = Cave::from(lines, has_floor);
    let print = can_fit_term(cave.width() * 2, cave.height() + 2);

    for gen in 0.. {
        if print {
            sleep(Duration::from_millis(13));
            cls();
        }
        if cave.is_still() {
            if !cave.spawn(SOURCE, Block::Sand { falling: true }) {
                break;
            }
        }
        let finish = cave.tick();
        if print {
            println!("{}\nGen: {}", cave, gen);
        }
        if finish {
            break;
        }
    }

    cave.blocks
        .values()
        .filter(|&block| Block::Sand { falling: false }.eq(block))
        .count()
}

fn main() {
    let input = read_input();
    let part1 = simulate(&input, false);
    println!("Part1: {}", part1);
    let part2 = simulate(&input, true);
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
