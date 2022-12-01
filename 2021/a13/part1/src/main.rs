use std::io::{self, BufRead};

#[derive(Debug)]
enum Fold {
    X(u16),
    Y(u16),
}

impl Fold {
    fn apply(&self, coords: &Vec<(u16, u16)>) -> Vec<(u16, u16)> {
        let mut new_coords = coords.iter().map(|coord| self.fold_coord(coord)).collect::<Vec<_>>();
        new_coords.sort();
        new_coords.dedup();
        new_coords
    }

    fn fold_coord(&self, coord: &(u16, u16)) -> (u16, u16) {
        match self {
            Fold::X(fold) => {
                let max = fold*2;
                if coord.0 > *fold {
                    (max - coord.0,coord.1)
                } else {
                    (coord.0, coord.1)
                }
            },
            Fold::Y(fold) => {
                let max = fold*2;
                if coord.1 > *fold {
                    (coord.0,max - coord.1)
                } else {
                    (coord.0, coord.1)
                }
            },
        }
    }
}

fn read_input() -> Result<(Vec<(u16, u16)>, Vec<Fold>), std::io::Error> {
    let mut coords = vec![];
    let mut folds = vec![];
    
    io::stdin().lock().lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .for_each(|line| {
            match line.chars().next().unwrap_or('\n') {
                'f' => folds.push(read_fold(&line)),
                x if x.is_numeric() =>  coords.push(read_coord(&line)),
                '\n' => (),
                _ => panic!("what is this!")
            }
        });

    return Ok((coords, folds));
}

fn plot(coords: &Vec<(u16, u16)>) -> Vec<Vec<bool>> {
    let max_row = coords.into_iter().map(|coord| coord.1 ).max().unwrap();
    let max_col = coords.into_iter().map(|coord| coord.0 ).max().unwrap();
    let mut board = vec![];
    let cols = (max_col + 1) as usize;
    for _ in 0..=max_row {
        board.push(vec![false; cols]);
    }
    for (x,y) in coords.iter() {
        board[*y as usize][*x as usize] = true;
    }
    board
}

fn print_board(board: &Vec<Vec<bool>>) {
    for row in board {
        for value in row {
            print!("{}", if *value { '#' } else { '.' });
        }
        print!("\n");
    }
}

fn read_fold(line: &str) -> Fold {
    let words = line.split(|c: char| !c.is_alphanumeric()).collect::<Vec<_>>();
    let value = u16::from_str_radix(words[3], 10).unwrap();
    match words[2] {
        "x" => Fold::X(value),
        "y" => Fold::Y(value),
        _ => panic!("unknown fold")
    }
}

fn read_coord(line: &str) -> (u16, u16) {
    let numbers: Vec<u16> = line.split(|c: char| !c.is_numeric() )
        .filter(|s| !s.is_empty() )
        .map(|s| u16::from_str_radix(s, 10).unwrap() )
        .collect();
    (numbers[0],numbers[1])
}

fn main() {
    let input = read_input().unwrap();

    let mut coords = input.0;
    for fold in input.1 {
        coords = fold.apply(&coords);
        println!("{} points left", coords.len());
    }
    
    print_board(&plot(&coords));
}
