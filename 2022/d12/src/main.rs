use std::{
    collections::HashMap,
    io::{self, BufRead},
};

type Pos = (i32, i32);
type Heightmap = Vec<Vec<i8>>;

fn read_input() -> (Heightmap, Pos, Pos) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    (
        io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .enumerate()
            .map(|(y, line)| read_line(&line, y, &mut start, &mut end))
            .collect(),
        start,
        end,
    )
}

fn read_line(line: &String, y: usize, start: &mut Pos, end: &mut Pos) -> Vec<i8> {
    line.bytes()
        .enumerate()
        .map(|(x, c)| match c {
            b'S' => {
                *start = (x as i32, y as i32);
                0i8
            }
            b'E' => {
                *end = (x as i32, y as i32);
                25i8
            }
            c => (c - b'a') as i8,
        })
        .collect()
}

fn heuristic_distance(from: &Pos, to: &Pos) -> i32 {
    let dx = (to.0 - from.0).abs();
    let dy = (to.1 - from.1).abs();
    return dx + dy;
}

fn height(map: &Heightmap, pos: &Pos) -> i8 {
    map[pos.1 as usize][pos.0 as usize]
}

fn weight(map: &Heightmap, from: &Pos, to: &Pos) -> i8 {
    assert!((from.0 - to.0).abs() <= 1 && (from.1 - to.1).abs() <= 1);
    height(map, to) - height(map, from)
}

fn map_size(map: &Heightmap) -> Pos {
    (map[0].len() as i32, map.len() as i32)
}

// A* translated and adapted from wikipedia's pseudocode
fn a_star(map: &Heightmap, from: &Pos, to: &Pos, part2: bool) -> Option<usize> {
    let size = map_size(map);
    let mut to_visit: Vec<Pos> = vec![*from];
    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, i32> = HashMap::new();
    g_score.insert(*from, 0);
    let mut f_score: HashMap<Pos, i32> = HashMap::new();
    f_score.insert(*from, heuristic_distance(from, to));

    while !to_visit.is_empty() {
        let current_idx = to_visit
            .iter()
            .enumerate()
            .min_by_key(|(_, pos)| f_score.get(pos))
            .unwrap()
            .0;
        let current = to_visit.remove(current_idx);

        if current.eq(to) {
            let mut pos = current;
            let mut count = 0;
            while prev.contains_key(&pos) {
                count += 1;
                pos = *prev.get(&pos).unwrap();
            }
            return Some(count);
        }

        for (neighbour, weight) in [
            (current.0 - 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
            (current.0 + 1, current.1),
        ]
        .iter()
        .filter(|&nb| within(nb, &size))
        .map(|&nb| (nb, weight(map, &current, &nb)))
        .filter(|(_, weight)| *weight <= 1)
        .filter(|(nb, _)| !part2 || height(map, nb) > 0)
        {
            let tentative_gscore = g_score.get(&current).unwrap() + 1 + (weight as i32);
            if tentative_gscore < g_score.get(&neighbour).map(|x| *x).unwrap_or(i32::MAX) {
                prev.insert(neighbour, current);
                g_score.insert(neighbour, tentative_gscore);
                f_score.insert(
                    neighbour,
                    tentative_gscore + heuristic_distance(&neighbour, to),
                );
                if !to_visit.contains(&neighbour) {
                    to_visit.push(neighbour);
                }
            }
        }
    }
    None
}

fn within(pos: &Pos, size: &Pos) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < size.0 && pos.1 < size.1
}

// breadth-first search from goal
fn part2(map: &Heightmap, to: &Pos) -> usize {
    let size = map_size(map);
    let mut to_visit: Vec<Pos> = vec![*to];
    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, i32> = HashMap::new();
    g_score.insert(*to, 0);

    while !to_visit.is_empty() {
        let current = to_visit.remove(0);

        if height(map, &current) == 0 {
            let mut pos = current;
            let mut count = 0;
            while prev.contains_key(&pos) {
                count += 1;
                pos = *prev.get(&pos).unwrap();
            }
            return count;
        }

        for (neighbour, weight) in [
            (current.0 - 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
            (current.0 + 1, current.1),
        ]
        .iter()
        .filter(|&nb| within(nb, &size))
        .map(|&nb| (nb, -weight(map, &current, &nb)))
        .filter(|(_, weight)| *weight <= 1)
        {
            let tentative_gscore = g_score.get(&current).unwrap() + 1 + (weight as i32);
            if tentative_gscore < g_score.get(&neighbour).map(|x| *x).unwrap_or(i32::MAX) {
                prev.insert(neighbour, current);
                g_score.insert(neighbour, tentative_gscore);
                if !to_visit.contains(&neighbour) {
                    to_visit.push(neighbour);
                }
            }
        }
    }
    panic!()
}

fn main() {
    let (map, from, to) = read_input();

    let part1 = a_star(&map, &from, &to, false).unwrap();
    println!("Part1 from {:?} to {:?}: {}", from, to, part1);

    let part2 = part2(&map, &to);
    println!("Part2: {}", part2);
}
