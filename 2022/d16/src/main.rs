use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self, BufRead},
    iter::once,
    str::FromStr,
};

use itertools::Itertools;

type ValveID = [u8; 2];
type Valves = HashMap<ValveID, Valve>;

struct Valve {
    id: ValveID,
    flow_rate: usize,
    tunnels: Vec<ValveID>,
}

fn valve_id(word: &str) -> ValveID {
    let bytes = word.as_bytes();
    assert_eq!(2, bytes.len());
    [bytes[0], bytes[1]]
}

fn valve_name(id: &ValveID) -> &str {
    std::str::from_utf8(id).unwrap()
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split(|c| c == ' ' || c == '=' || c == ',' || c == ';')
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(words[0], "Valve");
        let id = valve_id(words[1]);
        assert_eq!(words[2], "has");
        assert_eq!(words[3], "flow");
        assert_eq!(words[4], "rate");
        let flow_rate = words[5].parse::<usize>().unwrap();
        assert!(
            (words[6].eq("tunnels") && words[7].eq("lead") && words[9].eq("valves"))
                || (words[6].eq("tunnel") && words[7].eq("leads") && words[9].eq("valve"))
        );
        assert_eq!(words[8], "to");
        let tunnels = words[10..]
            .iter()
            .map(|word| valve_id(word))
            .collect::<Vec<_>>();
        assert!(tunnels.len() > 1 || words[9].eq("valve"));
        Ok(Valve {
            id,
            flow_rate,
            tunnels,
        })
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: flow_rate {}, tunnels: {}",
            valve_name(&self.id),
            self.flow_rate,
            self.tunnels.iter().map(|id| valve_name(&id)).join(",")
        )
    }
}

impl Valve {
    fn has_tunnel(&self, to: ValveID) -> bool {
        self.tunnels.contains(&to)
    }
}

fn read_input() -> Valves {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|ln| Valve::from_str(&ln).ok())
        .map(|valve| (valve.id, valve))
        .collect()
}

#[derive(Clone, Copy)]
enum Move {
    Go(ValveID, ValveID),
    Open(ValveID),
}

fn total_pressure_released(valves: &Valves, mut time: usize, mut path: &[Move]) -> usize {
    let mut total = 0;
    let mut pos = valve_id("AA");
    let mut flow = 0;
    path = &path[0..path.len().min(time as usize)];
    while path.len() > 0 {
        total += flow;
        time -= 1;
        if let Move::Go(next, _) = path[0] {
            assert!(valves.get(&pos).unwrap().has_tunnel(next));
            pos = next;
        } else if let Move::Open(valve) = path[0] {
            assert!(pos == valve);
            flow += valves.get(&pos).unwrap().flow_rate;
        }
        path = &path[1..];
    }
    total += flow * time;
    total
}

fn find_path(valves: &Valves, from: &ValveID, to: &ValveID) -> Vec<ValveID> {
    let mut to_visit: Vec<ValveID> = vec![*from];
    let mut prev: HashMap<ValveID, ValveID> = HashMap::new();
    while !to_visit.is_empty() {
        let current = valves.get(&to_visit.remove(0)).unwrap();
        for neighbour in current.tunnels.iter() {
            if neighbour.eq(to) {
                let mut path = vec![current.id, *to];
                let mut pos = current.id;
                while path[0].ne(from) {
                    pos = *prev.get(&pos).unwrap();
                    path.insert(0, pos);
                }
                return path;
            }
            if !prev.contains_key(neighbour) {
                prev.insert(*neighbour, current.id);
            }
            if !to_visit.contains(neighbour) {
                to_visit.push(*neighbour);
            }
        }
    }

    vec![]
}

type PathCache = HashMap<(ValveID, ValveID), Vec<ValveID>>;

fn find_cached_path<'a>(paths: &'a PathCache, from: &ValveID, to: &ValveID) -> &'a [ValveID] {
    paths.get(&(*from, *to)).unwrap_or_else(|| {
        panic!(
            "no path between {} and {}",
            valve_name(from),
            valve_name(to)
        )
    })
}

fn cache_paths(valves: &Valves, between: &Vec<(ValveID, ValveID)>) -> PathCache {
    let mut cache = HashMap::new();
    for (from, to) in between {
        cache.insert((*from, *to), find_path(valves, from, to));
    }
    cache
}

fn find_best_part(
    valves: &Valves,
    paths: &PathCache,
    from: &ValveID,
    time: usize,
    flow: usize,
    acc_flow: usize,
    left_to_open: Vec<ValveID>,
) -> usize {
    if time == 0 {
        return acc_flow;
    } else if left_to_open.is_empty() {
        return acc_flow + flow * time;
    }

    let mut max = acc_flow + flow * time;
    for next in left_to_open.iter() {
        // is path feasible?
        let path = find_cached_path(paths, from, &next);
        if path.len() > time {
            continue;
        }
        // take path
        let valve_flow = valves.get(next).unwrap().flow_rate;
        let next_left_to_open = left_to_open
            .iter()
            .filter(|&v| v != next)
            .map(|v| *v)
            .collect::<Vec<_>>();
        let next_max = find_best_part(
            valves,
            paths,
            &next,
            time - path.len(),
            flow + valve_flow,
            acc_flow + flow * path.len(),
            next_left_to_open,
        );
        if next_max > max {
            max = next_max;
        }
    }
    max
}

fn part1(valves: &Valves, time: usize) -> usize {
    let left_to_open = valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id)
        .collect::<Vec<_>>();
    let path_cache = cache_paths(
        valves,
        &valves
            .keys()
            .permutations(2)
            .map(|p| (*p[0], *p[1]))
            .collect::<Vec<_>>(),
    );
    let from = valve_id("AA");

    find_best_part(valves, &path_cache, &from, time, 0, 0, left_to_open)
}

#[derive(Clone)]
struct Actor<'a> {
    pos: ValveID,
    path: &'a [ValveID],
    moves: Vec<Move>,
}

impl<'a> Actor<'a> {
    fn next(&self) -> Actor {
        if self.path.is_empty() {
            return self.clone();
        }
        Actor {
            pos: self.path[0],
            path: &self.path[1..],
            moves: self
                .moves
                .iter()
                .map(|m| *m)
                .chain(once(Move::Go(self.path[0], *self.path.last().unwrap())))
                .collect(),
        }
    }

    fn destination(&self) -> Option<ValveID> {
        self.path.last().map(|i| *i)
    }
}

fn find_best_2<'a>(
    valves: &Valves,
    paths: &'a PathCache,
    actors: [Actor<'a>; 2],
    step: usize,
    time: usize,
    opening: &Vec<ValveID>,
    flow: usize,
    acc_flow: usize,
    left_to_open: &Vec<ValveID>,
) -> (usize, Vec<Move>, Vec<Move>) {
    if step == 2 || (step == 1 && actors[1].pos == [0, 0]) {
        // next step
        if time == 0 {
            assert!(opening.is_empty());
            return (acc_flow, actors[0].moves.to_vec(), actors[1].moves.to_vec());
        } else if left_to_open.is_empty() {
            assert!(opening.is_empty());
            return (
                acc_flow + flow * time,
                actors[0].moves.to_vec(),
                actors[1].moves.to_vec(),
            );
        }

        let opening_flow: usize = opening
            .iter()
            .map(|v| valves.get(v).unwrap())
            .map(|v| v.flow_rate)
            .sum();

        return find_best_2(
            valves,
            paths,
            actors,
            0,
            time - 1,
            &vec![],
            flow + opening_flow,
            acc_flow + flow,
            &left_to_open
                .iter()
                .filter(|&v| !opening.contains(v))
                .map(|v| *v)
                .collect::<Vec<_>>(),
        );
    }

    let actor = actors[step].clone();
    if actor.path.is_empty() {
        // reached end of path
        if left_to_open.contains(&actor.pos) && !opening.contains(&actor.pos) {
            // open valve
            let next_actor = Actor {
                pos: actor.pos,
                path: actor.path,
                moves: actor
                    .moves
                    .iter()
                    .map(|m| *m)
                    .chain(once(Move::Open(actor.pos)))
                    .collect_vec(),
            };
            return find_best_2(
                valves,
                paths,
                match step {
                    0 => [next_actor, actors[1].clone()],
                    1 => [actors[0].clone(), next_actor],
                    _ => panic!(),
                },
                step + 1,
                time,
                &opening.iter().chain(once(&actor.pos)).map(|v| *v).collect(),
                flow,
                acc_flow,
                left_to_open,
            );
        } else {
            // already opened - chose new path
            let mut max = (
                acc_flow + flow * time,
                actors[0].moves.to_vec(),
                actors[1].moves.to_vec(),
            );
            let from = &actor.pos;
            for next in left_to_open.into_iter().filter(|&&v| {
                v != *from
                    && Some(v) != actors[0].destination()
                    && Some(v) != actors[1].destination()
            }) {
                // is path feasible?
                let path = find_cached_path(paths, from, &next);
                if path.len() > time {
                    continue;
                }
                // take path
                let next_actor = Actor {
                    pos: path[1],
                    path: &path[2..],
                    moves: actor
                        .moves
                        .iter()
                        .map(|m| *m)
                        .chain(once(Move::Go(path[1], *path.last().unwrap())))
                        .collect_vec(),
                };
                let next_max = find_best_2(
                    valves,
                    paths,
                    match step {
                        0 => [next_actor, actors[1].clone()],
                        1 => [actors[0].clone(), next_actor],
                        _ => panic!(),
                    },
                    step + 1,
                    time,
                    opening,
                    flow,
                    acc_flow,
                    left_to_open,
                );
                if next_max.0 > max.0 {
                    max = next_max;
                }
            }
            return max;
        }
    } else {
        // move to next step
        return find_best_2(
            valves,
            paths,
            match step {
                0 => [actor.next(), actors[1].clone()],
                1 => [actors[0].clone(), actor.next()],
                _ => panic!(),
            },
            step + 1,
            time,
            opening,
            flow,
            acc_flow,
            left_to_open,
        );
    }
}

fn part2(valves: &Valves, time: usize) -> usize {
    let left_to_open = valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id)
        .collect::<Vec<_>>();
    let path_cache = cache_paths(
        valves,
        &valves
            .keys()
            .permutations(2)
            .map(|p| (*p[0], *p[1]))
            .collect::<Vec<_>>(),
    );
    let from = valve_id("AA");

    let (max, m0, m1) = find_best_2(
        valves,
        &path_cache,
        [
            Actor {
                pos: from,
                path: &[],
                moves: vec![],
            },
            Actor {
                pos: from,
                path: &[],
                moves: vec![],
            },
        ],
        0,
        time,
        &vec![],
        0,
        0,
        &left_to_open,
    );

    print_paths(&[&m1, &m0]);
    return max;
}

fn print_paths(path: &[&Vec<Move>]) {
    for i in 0..path[0].len() {
        println!("Minute {}", i + 1);
        for p in 0..path.len() {
            let (m, x) = match &path[p][i] {
                Move::Go(v, dst) => (
                    "move to",
                    format!("{} for {}", valve_name(v), valve_name(dst)),
                ),
                Move::Open(v) => ("open", String::from(valve_name(v))),
            };
            println!("{} {} {}", if p == 0 { "you" } else { "elephant" }, m, x);
        }
    }
}

fn main() {
    let valves = read_input();

    println!("Part1: {}", part1(&valves, 30));
    println!("Part2: {}", part2(&valves, 26));
}
