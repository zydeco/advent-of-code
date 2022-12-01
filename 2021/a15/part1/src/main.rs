use std::io::{self, BufRead};
use std::collections::{HashMap,HashSet};

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

fn next_coord(queue: &HashSet<Coord>, dist: &HashMap<Coord, u32>) -> Coord {
    *dist.iter().filter(|(k,_)| queue.contains(k)).min_by_key(|(_,v)| *v).unwrap().0
}

fn shortest_path(map: &Vec<Vec<u8>>) -> u32 {
    let mut queue: HashSet<Coord> = HashSet::new();
    let mut dist: HashMap<Coord, u32> = HashMap::new();
    let mut prev: HashMap<Coord, Coord> = HashMap::new();
    
    let rows = map.len();
    let cols = map[0].len();
    for y in 0..rows {
        for x in 0..cols {
            let coord: Coord = (x as i16,y as i16);
            dist.insert(coord, u32::MAX);
            queue.insert(coord);
        }
    }

    let source: Coord = (0,0);
    dist.insert(source, 0);

    while !queue.is_empty() {
        let u = next_coord(&queue, &dist);
        queue.remove(&u);
        for v in [
            (u.0 - 1, u.1),
            (u.0, u.1 - 1),
            (u.0 + 1, u.1),
            (u.0, u.1 + 1)
        ].into_iter().filter(|v| queue.contains(v)) {
            let this_dist = map[v.1 as usize][v.0 as usize] as u32;
            let alt = dist.get(&u).unwrap() + this_dist;
            if alt < *dist.get(&v).unwrap() {
                dist.insert(v, alt);
                prev.insert(v, u);
            }
        }
    }

    return *dist.get(&(cols as i16 - 1, rows as i16 - 1)).unwrap();
}

fn main() {
    let map = read_input();
    let result = shortest_path(&map);
    println!("Hello, {}", result);
}
