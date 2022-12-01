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

fn adjacents(heightmap: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
    let cols = heightmap[0].len() as i32;
    let rows = heightmap.len() as i32;
    let mut result = vec![];
    for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)] {
        let x = row as i32 + dx;
        let y = col as i32 + dy;
        if x >= 0 && y >= 0 && x < rows && y < cols {
            result.push(heightmap[x as usize][y as usize]);
        }
    }
    result
}

fn is_low_point(heightmap: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let adj = adjacents(heightmap, row, col);
    let value = heightmap[row as usize][col as usize];
    let min_adj = *adj.iter().min().unwrap();
    value < min_adj
}

fn basin_size(heightmap: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let cols = heightmap[0].len() as i32;
    let rows = heightmap.len() as i32;
    let mut map2 = heightmap.clone();
    let mut points = vec![(row, col)];
    let mut count = 0;
    while points.len() > 0 {
        let next_point = points.pop().unwrap();
        for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)] {
            let x = next_point.0 as i32 + dx;
            let y = next_point.1 as i32 + dy;
            if x >= 0 && y >= 0 && x < rows && y < cols && map2[x as usize][y as usize] < 9 {
                count += 1;
                map2[x as usize][y as usize] = 9;
                points.push((x as usize, y as usize))
            }
        }
    }
    count
}

fn main() {
    let hmap = read_input();
    let mut total = 0u32;
    let mut low_points = vec![];
    for row in 0..hmap.len() {
        for col in 0..hmap[0].len() {
            if is_low_point(&hmap, row, col) {
                let risk = 1 + hmap[row][col] as u32;
                total += risk;
                low_points.push((row, col))
            }
        }
    }
    println!("part1: is {}!", total);
    
    let mut basins = vec![];
    for p in low_points {
        let bs = basin_size(&hmap, p.0, p.1);
        basins.push(bs);
        //println!("basin at {}, {}: {}", p.0, p.1, bs);
    }
    basins.sort();
    let result = basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap();
    println!("part2: {}", result);
}
