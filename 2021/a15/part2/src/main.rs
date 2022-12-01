use std::io::{self, BufRead};

fn read_line(buf: &String) -> Vec<u8> {
    buf
        .chars()
        .map(|c| c.to_digit(10) )
        .filter(Option::is_some)
        .map(|c| c.unwrap() as u8)
        .collect::<Vec<_>>()
}

fn read_input() -> Vec<Vec<u8>> {
    io::stdin().lock().lines()
        .map(|line| read_line(&line.ok().unwrap()) )
        .collect()
}

type Coord = (i16,i16);

fn coord_idx(coord: &Coord, cols: usize) -> usize {
    coord.0 as usize * cols + coord.1 as usize
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<u8>>) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            print!("{}", match board[y][x] {
                0 => '.',
                1..=9 => (0x30 + board[y][x]) as char,
                _ => 'X',
            })
        }
        print!("\n")
    }
}

fn add_wrap(x: u8, y: u8) -> u8 {
    let z = x + y;
    if z > 9 {
        z - 9
    } else {
        z
    }
}

fn expand(map: &Vec<Vec<u8>>, times: usize) -> Vec<Vec<u8>> {
    let mut new_map = vec![];

    // expand horizontally
    for r in 0..map.len() {
        let row = &map[r];
        let mut new_row = Vec::with_capacity(row.len() * times);
        for n in 0..times {
            for c in 0..row.len() {
                new_row.push(add_wrap(row[c], n as u8));
            }
        }
        new_map.push(new_row);
    }

    // expand vertically
    for n in 1..times {
        for r in 0..map.len() {
            let row = &new_map[r];
            let new_row = row.into_iter().map(|x| add_wrap(*x, n as u8)).collect::<Vec<_>>();
            new_map.push(new_row);
        }
    }

    new_map
}

fn is_valid_coord(coord: &Coord, size: (usize, usize)) -> bool {
    coord.0 >= 0 && coord.1 >= 0 && coord.0 < size.0 as i16 && coord.1 < size.1 as i16
}

fn shortest_path(map: &Vec<Vec<u8>>) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut queue = Vec::with_capacity(rows * cols);

    let mut dist = vec![u32::MAX; rows * cols];
    dist[0] = 0;
    let src: Coord = (0, 0);
    let dst: Coord = (cols as i16 - 1, rows as i16 - 1);
    queue.push(src);

    while !queue.is_empty() {
        let (idx,&u) = queue.iter().enumerate().min_by_key(|(_,coord)| dist[coord_idx(coord, cols)]).unwrap();
        queue.remove(idx);
        for v in [
            (u.0 - 1, u.1),
            (u.0, u.1 - 1),
            (u.0 + 1, u.1),
            (u.0, u.1 + 1)
        ].into_iter().filter(|v| is_valid_coord(v, (cols,rows)) ) {
            let this_dist = map[v.1 as usize][v.0 as usize] as u32;
            let alt = this_dist + dist[coord_idx(&u, cols)];
            let cur_dist = dist[coord_idx(&v, cols)];
            if alt < cur_dist {
                if cur_dist == u32::MAX {
                    queue.push(v);
                }
                dist[coord_idx(&v, cols)] = alt;
            }
        }
    }

    return dist[coord_idx(&dst, cols)];
}

fn main() {
    let input = read_input();
    let map = expand(&input, 5);
    let result = shortest_path(&map);
    println!("Hello, {}", result);
}
