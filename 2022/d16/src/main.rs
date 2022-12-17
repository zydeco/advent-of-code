use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self, BufRead},
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

enum Move {
    Go(ValveID),
    Open(ValveID),
}

macro_rules! go {
    ($vid:ident) => {
        Move::Go(valve_id(stringify!($vid)))
    };
}

macro_rules! open {
    ($vid:ident) => {
        Move::Open(valve_id(stringify!($vid)))
    };
}

fn total_pressure_released(valves: &Valves, mut time: usize, mut path: &[Move]) -> usize {
    let mut total = 0;
    let mut pos = valve_id("AA");
    let mut flow = 0;
    path = &path[0..path.len().min(time as usize)];
    while path.len() > 0 {
        total += flow;
        time -= 1;
        if let Move::Go(next) = path[0] {
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

fn path_opening(
    valves: &Valves,
    cache: &PathCache,
    mut opening: &[ValveID],
    mut from: ValveID,
    max_steps: usize,
) -> Vec<Move> {
    let mut moves = vec![];
    while opening.len() > 0 && moves.len() < max_steps {
        let to = opening[0];
        for i in find_cached_path(cache, &from, &to).iter().skip(1) {
            moves.push(Move::Go(*i));
        }
        moves.push(Move::Open(to));
        from = to;
        opening = &opening[1..];
    }

    moves
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

fn print_path(path: &Vec<Move>) {
    for m in path {
        let (m, x) = match m {
            Move::Go(x) => ("Move", valve_name(x)),
            Move::Open(x) => ("Open", valve_name(x)),
        };
        println!("{}({})", m, x);
    }
}

fn main() {
    let valves = read_input();

    println!("Part1: {}", part1(&valves, 30));
}
