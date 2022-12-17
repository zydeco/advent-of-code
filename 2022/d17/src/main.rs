use std::{
    io,
    ops::{BitAnd, BitOr, BitOrAssign, RangeInclusive, Shl, Shr},
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn read_input() -> Vec<Direction> {
    io::stdin()
        .lines()
        .filter_map(Result::ok)
        .next()
        .unwrap()
        .bytes()
        .map(|b| match b {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!(),
        })
        .collect()
}

struct Shape {
    bits: [u8; 4],
}

const SHAPES: [Shape; 5] = [
    Shape {
        // flat
        bits: [0b0011110, 0, 0, 0],
    },
    Shape {
        // cross
        bits: [0b0001000, 0b0011100, 0b0001000, 0],
    },
    Shape {
        // angled
        bits: [0b0011100, 0b0000100, 0b0000100, 0],
    },
    Shape {
        // tall
        bits: [0b0010000, 0b0010000, 0b0010000, 0b0010000],
    },
    Shape {
        // square
        bits: [0b0011000, 0b0011000, 0, 0],
    },
];

#[derive(Default)]
struct FallingShape {
    bits: [u8; 4],
    height: usize,
    shape_height: usize,
}

struct Field {
    rows: Vec<u8>,
}

impl Field {
    fn new() -> Field {
        Field {
            rows: Vec::with_capacity(128),
        }
    }

    fn row(&self, row: usize) -> u8 {
        *self.rows.get(row).unwrap_or(&0)
    }

    fn height(&self) -> usize {
        self.rows
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.rows.len())
    }

    fn rest(&mut self, shape: &FallingShape) {
        if self.rows.len() <= shape.top() {
            self.rows.resize(shape.top() + 1, 0);
        }
        let h = shape.height;
        for i in 0..shape.shape_height {
            self.rows[h + i].bitor_assign(shape.bits[i]);
        }
    }

    fn can_fit(&self, shape: &[u8; 4], row: usize) -> bool {
        for r in 0..4 {
            if shape[r].bitand(self.row(row + r)) != 0 {
                return false;
            }
        }
        true
    }

    fn find_cycle(&self) -> Option<RangeInclusive<usize>> {
        let height = self.height();
        if height < 20 {
            return None;
        }

        let search_top = height - 20;
        for len in 10..search_top / 2 {
            let start_top = search_top - len;
            let top = &self.rows.as_slice()[start_top..start_top + len];
            let bottom = &self.rows.as_slice()[start_top - len..start_top];
            if top == bottom {
                return Some(start_top..=start_top + len - 1);
            }
        }
        None
    }
}

impl FallingShape {
    fn spawn(shape: &Shape, field: &Field) -> FallingShape {
        let height = 3 + field.height();
        let shape_height = shape.bits.iter().filter(|&&b| b != 0).count();
        FallingShape {
            bits: shape.bits,
            height,
            shape_height,
        }
    }

    fn row_at_height(&self, height: usize) -> u8 {
        if height < self.height || height > self.height + 3 {
            0
        } else {
            self.bits[height - self.height]
        }
    }

    fn top(&self) -> usize {
        if self.shape_height == 0 {
            return 0;
        }
        self.height + self.shape_height - 1
    }

    fn fall(&mut self, field: &Field) -> bool {
        if self.height == 0 {
            return false;
        }
        if self.height == 0 || !field.can_fit(&self.bits, self.height - 1) {
            false
        } else {
            self.height -= 1;
            true
        }
    }

    fn push(&mut self, direction: &Direction, field: &Field) -> bool {
        match direction {
            Direction::Left => self.push_left(field),
            Direction::Right => self.push_right(field),
        }
    }

    fn push_left(&mut self, field: &Field) -> bool {
        // shifted bits
        let bits: [u8; 4] = [
            self.bits[0].shl(1),
            self.bits[1].shl(1),
            self.bits[2].shl(1),
            self.bits[3].shl(1),
        ];

        // bump into wall?
        if bits[0]
            .bitor(bits[1])
            .bitor(bits[2])
            .bitor(bits[3])
            .bitand(0b10000000u8)
            != 0
        {
            return false;
        }

        if !field.can_fit(&bits, self.height) {
            return false;
        }

        self.bits = bits;
        true
    }

    fn push_right(&mut self, field: &Field) -> bool {
        // bump into wall?
        if self.bits[0]
            .bitor(self.bits[1])
            .bitor(self.bits[2])
            .bitor(self.bits[3])
            .bitand(0b1u8)
            != 0
        {
            return false;
        }

        // shifted bits
        let bits: [u8; 4] = [
            self.bits[0].shr(1),
            self.bits[1].shr(1),
            self.bits[2].shr(1),
            self.bits[3].shr(1),
        ];

        if !field.can_fit(&bits, self.height) {
            return false;
        }

        self.bits = bits;
        true
    }
}

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_field(field: &Field, falling: &FallingShape) {
    fn bit_to_char(field_byte: u8, shape_byte: u8, mask: u8) -> char {
        if shape_byte.bitand(mask) != 0 {
            '@'
        } else if field_byte.bitand(mask) != 0 {
            '#'
        } else {
            '.'
        }
    }

    let top = falling.top().max(field.height());
    for h in (0..=top).rev() {
        let f = field.row(h);
        let s = falling.row_at_height(h);
        println!(
            "|{}{}{}{}{}{}{}|",
            bit_to_char(f, s, 0b1000000),
            bit_to_char(f, s, 0b100000),
            bit_to_char(f, s, 0b10000),
            bit_to_char(f, s, 0b1000),
            bit_to_char(f, s, 0b100),
            bit_to_char(f, s, 0b10),
            bit_to_char(f, s, 0b1)
        )
    }
    println!("+-------+");
}

fn main() {
    let input = read_input();
    let mut shapes = SHAPES.iter().cycle();
    let mut jet = input.iter().cycle();
    let mut field = Field::new();
    let mut fallen = 0;

    // first rock
    let mut falling = FallingShape::spawn(shapes.next().unwrap(), &field);
    falling.push(&jet.next().unwrap(), &field);

    let mut next_cycle: Option<usize> = None;
    let mut cycle_size: Option<usize> = None;
    let mut cycle_fallen: Option<usize> = None;
    let mut skipped_height = 0;

    for dir in jet {
        // fall
        if !falling.fall(&field) {
            field.rest(&falling);
            fallen += 1;
            // spawn
            if fallen == 2022 {
                println!("Height after 2022 is {}", field.height());
            }
            if fallen == 1000000000000 {
                println!(
                    "Height after 1000000000000 is {}",
                    field.height() + skipped_height
                );
                break;
            }
            falling = FallingShape::spawn(shapes.next().unwrap(), &field);
        }

        if next_cycle.is_none() {
            // find first cycle
            if let Some(cycle) = field.find_cycle() {
                cycle_size = Some(cycle.end() - cycle.start() + 1);
                println!("Cycle of size {} found at {:?}", cycle_size.unwrap(), cycle);
                cycle_fallen = Some(fallen);
                next_cycle = Some(field.height() + cycle_size.unwrap());
            }
        } else if next_cycle == Some(field.height()) && skipped_height == 0 {
            // fast forward
            let fallen_per_cycle = fallen - cycle_fallen.unwrap();
            let height_per_cycle = cycle_size.unwrap();
            let cycles_needed = (1000000000000 - fallen) / fallen_per_cycle;
            fallen += cycles_needed * fallen_per_cycle;
            skipped_height = cycles_needed * height_per_cycle;

            println!("Skipped height: {}", skipped_height);
        }

        // push
        falling.push(dir, &field);
    }
}
