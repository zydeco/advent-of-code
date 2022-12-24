use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    ops::Add,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
struct Pos {
    x: i32,
    y: i32,
}

macro_rules! pos {
    ($a:expr,$b:expr) => {{
        Pos {
            x: $a as i32,
            y: $b as i32,
        }
    }};
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Blizzard {
    pos: Pos,
    dir: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Valley {
    entry: Pos,
    exit: Pos,
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            '^' => Ok(Self::Up),
            _ => Err(value),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        pos!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Pos {
    fn taxicab_distance(self, other: Self) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs()) as usize
    }
}

// (entry, blizzards, exit)
fn read_input() -> (Valley, Vec<Blizzard>) {
    let mut blizzards = vec![];
    let mut lines = io::stdin().lock().lines().filter_map(Result::ok);
    let width = lines.next().unwrap().len() - 2;
    let mut y = 0;
    for line in lines {
        for (x, c) in line.chars().enumerate() {
            assert!(x > 0 || c == '#', "Lines start with wall");
            assert!(x <= width || c == '#', "Lines end in wall");
            if let Some(dir) = c.try_into().ok() {
                let pos = pos!(x - 1, y);
                blizzards.push(Blizzard { pos, dir });
            }
        }
        y += 1;
    }
    (
        Valley {
            entry: pos!(0, -1),
            exit: pos!(width - 1, y - 1),
        },
        blizzards,
    )
}

impl Valley {
    fn print(&self, blizzards: &Vec<Blizzard>, position: &Pos) {
        println!(
            "#{:#<width$}",
            if self.entry.eq(position) { 'E' } else { '.' },
            width = 2 + self.exit.x as usize
        );
        let (width, height) = self.size();
        let blizzards_at = blizzards_by_position(&mut blizzards.iter());
        assert!(
            blizzards_at.get(position).is_none(),
            "All your elves are dead."
        );
        for y in 0..height {
            print!("#");
            for x in 0..width {
                let cur_pos = pos!(x, y);
                print!(
                    "{}",
                    match blizzards_at.get(&cur_pos) {
                        None if cur_pos.eq(position) => 'E',
                        None => '.',
                        Some(b) if b.len() == 1 => b[0].into(),
                        Some(b) if b.len() < 10 => char::from_digit(b.len() as u32, 10).unwrap(),
                        Some(_) => '*',
                    }
                )
            }
            println!("#");
        }
        println!(
            "{:#>width$}#",
            if self.exit.eq(position) { 'E' } else { '.' },
            width = 2 + self.exit.x as usize
        );
    }

    fn size(&self) -> (i32, i32) {
        (self.exit.x + 1, self.exit.y)
    }

    fn cycle_blizzards(&self, blizzards: &Vec<Blizzard>) -> Vec<Blizzard> {
        let (width, height) = self.size();
        blizzards.iter().map(|b| b.next(width, height)).collect()
    }

    // positions of all blizzards at any time, in repeating cycle
    fn full_cycle(&self, blizzards: &Vec<Blizzard>) -> Vec<HashSet<Pos>> {
        let mut all = vec![blizzards_positions(blizzards)];
        let mut new_blizzards = self.cycle_blizzards(blizzards);
        while new_blizzards.ne(blizzards) {
            all.push(blizzards_positions(&new_blizzards));
            new_blizzards = self.cycle_blizzards(&new_blizzards);
        }
        all
    }

    fn find_path(&self, initial_blizzards: &Vec<Blizzard>, from: Pos, to: Pos) -> usize {
        let (width, height) = self.size();
        let range_x = 0..width;
        let range_y = 0..height;
        let all_blizzards = self.full_cycle(initial_blizzards);
        let num_blizzards = all_blizzards.len();

        // position in time
        type PosT = (Pos, usize);
        let mut to_visit: Vec<PosT> = vec![(from, 0)];
        let mut prev: HashMap<PosT, PosT> = HashMap::new();
        let mut max_queue = 1;

        while !to_visit.is_empty() {
            let current_idx = to_visit
                .iter()
                .enumerate()
                // wrong estimation, found path might be too long
                // going breadth-first is too slow on the real input - see other solution
                .min_by_key(|(_, (pos, _))| pos.taxicab_distance(to))
                .unwrap()
                .0;
            let current = to_visit.remove(current_idx);

            if current.0.eq(&to) {
                return current.1;
                /* reconstruct path
                let mut pos = current;
                let mut count = 0;
                while prev.contains_key(&pos) {
                    count += 1;
                    pos = *prev.get(&pos).unwrap();
                }
                return count; */
            }

            for next in [
                (current.0 + Direction::Down.into(), current.1 + 1),
                (current.0 + Direction::Right.into(), current.1 + 1),
                (current.0 + Direction::Left.into(), current.1 + 1),
                (current.0 + Direction::Up.into(), current.1 + 1),
                (current.0, current.1 + 1),
            ]
            .iter()
            .filter(|(pos, _)| {
                {
                    // next is destination, or is inside valley
                    to.eq(pos)
                        || (range_x.contains(&pos.x) && range_y.contains(&pos.y))
                        || (from.eq(pos) && current.0.eq(pos))
                }
            })
            .filter(|(pos, gen)| {
                // next is free
                let blizzards = &all_blizzards[*gen % num_blizzards];
                !blizzards.contains(pos)
            }) {
                prev.insert(*next, current);
                to_visit.push(*next);
                max_queue = max_queue.max(to_visit.len());
            }
        }
        panic!("No path found MQ={}", max_queue);
    }

    fn shortest_path(&self, initial_blizzards: &Vec<Blizzard>, from: Pos, to: Pos) -> usize {
        let all_blizzards = self.full_cycle(initial_blizzards);
        self.shortest_path_from_state(&all_blizzards, from, to, 0)
    }

    fn shortest_path_3(&self, initial_blizzards: &Vec<Blizzard>, from: Pos, to: Pos) -> usize {
        let all_blizzards = self.full_cycle(initial_blizzards);
        let a = self.shortest_path_from_state(&all_blizzards, from, to, 0);
        let b = self.shortest_path_from_state(&all_blizzards, to, from, a);
        let c = self.shortest_path_from_state(&all_blizzards, from, to, a + b);
        a + b + c
    }

    // find shortest path without calculating the actual path
    fn shortest_path_from_state(
        &self,
        all_blizzards: &Vec<HashSet<Pos>>,
        from: Pos,
        to: Pos,
        initial: usize,
    ) -> usize {
        let (width, height) = self.size();
        let range_x = 0..width;
        let range_y = 0..height;
        let num_blizzards = all_blizzards.len();
        let mut reached: HashSet<Pos> = [from].into_iter().collect();
        let mut step = initial + 1;
        while !reached.contains(&to) {
            let mut next_reached: HashSet<Pos> = HashSet::new();
            for current in reached.iter() {
                for next in [
                    *current + Direction::Down.into(),
                    *current + Direction::Right.into(),
                    *current + Direction::Left.into(),
                    *current + Direction::Up.into(),
                    *current,
                ]
                .iter()
                .filter(|pos| {
                    {
                        // next is destination, or is inside valley
                        to.eq(pos)
                            || (range_x.contains(&pos.x) && range_y.contains(&pos.y))
                            || from.eq(pos)
                    }
                })
                .filter(|pos| {
                    // next is free
                    let blizzards = &all_blizzards[step % num_blizzards];
                    !blizzards.contains(pos)
                }) {
                    next_reached.insert(*next);
                }
            }
            reached = next_reached;
            step += 1;
        }

        step - 1 - initial
    }
}

impl Blizzard {
    fn next(&self, width: i32, height: i32) -> Blizzard {
        Blizzard {
            pos: self.pos.wrapping_add(&self.dir.into(), width, height),
            dir: self.dir,
        }
    }
}

impl Into<Pos> for Direction {
    fn into(self) -> Pos {
        match self {
            Direction::Up => pos!(0, -1),
            Direction::Down => pos!(0, 1),
            Direction::Left => pos!(-1, 0),
            Direction::Right => pos!(1, 0),
        }
    }
}

impl Pos {
    fn wrapping_add(&self, rhs: &Pos, width: i32, height: i32) -> Pos {
        pos!(
            (self.x + rhs.x).rem_euclid(width),
            (self.y + rhs.y).rem_euclid(height)
        )
    }
}

fn blizzards_by_position<'a>(
    blizzards: &mut dyn Iterator<Item = &'a Blizzard>,
) -> HashMap<Pos, Vec<Direction>> {
    let mut map: HashMap<Pos, Vec<Direction>> = HashMap::new();
    for b in blizzards {
        map.entry(b.pos).or_default().push(b.dir)
    }
    map
}

fn blizzards_positions(blizzards: &Vec<Blizzard>) -> HashSet<Pos> {
    blizzards.iter().map(|b| b.pos).collect()
}

fn main() {
    let (valley, blizzards) = read_input();
    valley.print(&blizzards, &valley.entry);

    println!(
        "Path: {}",
        valley.shortest_path(&blizzards, valley.entry, valley.exit)
    );

    println!(
        "Path back and forth and back again: {}",
        valley.shortest_path_3(&blizzards, valley.entry, valley.exit)
    );
}
