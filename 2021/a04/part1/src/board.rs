use std::convert::TryInto;

#[derive(Clone, Copy)]
pub struct Board {
    pub values: [u8; Board::SIZE*Board::SIZE],
    pub marked: u32,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for row in 0..Board::SIZE {
            let line = &self.values[Board::SIZE*row..(Board::SIZE*row)+Board::SIZE];
            for col in 0..Board::SIZE {
                let last = if col == Board::SIZE-1 && row < Board::SIZE-1 { "\n" } else if col < Board::SIZE-1 { " " } else { "" };
                if self.is_marked(row, col) {
                    write!(f, "\x1b[7m{x:>w$}\x1b[0m{sep}", w=2,x=line[col], sep=last)?
                } else {
                    write!(f, "{x:>w$}{sep}", w=2,x=line[col], sep=last)?
                }
            }
        }
        Ok(())
    }
}

impl Board {
    pub const SIZE: usize = 5; // rows & columns
    // TODO: compile-time check that marked has enough bits

    pub fn new(values: &[u8]) -> Board {
        return Board {
            values: values.try_into().expect("Wrong number of numbers"),
            marked: 0u32
        }
    }

    fn is_marked(&self, row: usize, col: usize) -> bool {
        self.marked & (1 << (col + Board::SIZE*row)) != 0
    }

    pub fn call(&mut self, number: u8) {
        for n in 0..(Board::SIZE * Board::SIZE) {
            if self.values[n] == number {
                self.marked |= 1 << n;
            }
        }
    }

    pub fn is_winning(&self) -> bool {
        if (self.marked.count_ones() as usize) < Board::SIZE {
            return false;
        }
        for i in 0..Board::SIZE {
            let row_mask = 0b11111 << (Board::SIZE*i);
            if self.marked & row_mask == row_mask {
                return true;
            }
            let col_mask = 0b00001_00001_00001_00001_00001 << i;
            if self.marked & col_mask == col_mask {
                return true;
            }
        }
        return false;
    }

    pub fn score(&self) -> u32 {
        let mut score = 0u32;
        for n in 0..(Board::SIZE * Board::SIZE) {
            if self.marked & (1 << n) == 0 {
                score += self.values[n] as u32;
            }
        }
        return score;
    }
}