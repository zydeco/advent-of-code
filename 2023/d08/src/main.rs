use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, BufRead},
    iter,
};

use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

type NodeID = [u8; 3];
type Node = (NodeID, NodeID);

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("Invalid direction {}", c),
        }
    }

    fn from(&self, node: &Node) -> NodeID {
        match self {
            Direction::L => node.0,
            Direction::R => node.1,
        }
    }
}

fn node_id(b: &[u8]) -> NodeID {
    [b[0], b[1], b[2]]
}

fn print_nodes(nodes: &HashMap<NodeID, Node>) {
    for (node, (left, right)) in nodes {
        println!(
            "{}{}{} = ({}{}{}, {}{}{})",
            node[0] as char,
            node[1] as char,
            node[2] as char,
            left[0] as char,
            left[1] as char,
            left[2] as char,
            right[0] as char,
            right[1] as char,
            right[2] as char
        )
    }
}

fn read_input() -> (Vec<Direction>, HashMap<NodeID, Node>) {
    let mut lines = io::stdin().lock().lines().filter_map(Result::ok);
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect();
    lines.next();
    let nodes = lines
        .map(|line| {
            assert_eq!(16, line.len());
            let lb = line.as_bytes();
            let key: NodeID = node_id(lb);
            let left: NodeID = node_id(&lb[7..]);
            let right: NodeID = node_id(&lb[12..]);
            (key, (left, right))
        })
        .collect();
    (directions, nodes)
}

fn part1(directions: &Vec<Direction>, nodes: &HashMap<NodeID, Node>) -> Option<usize> {
    let start = node_id("AAA".as_bytes());
    let end = node_id("ZZZ".as_bytes());
    let mut steps = 0;
    let mut pos = start;
    if !(nodes.contains_key(&start) && nodes.contains_key(&end)) {
        return None;
    }
    for dir in iter::repeat(directions).flat_map(|f| f) {
        pos = dir.from(&nodes[&pos]);
        steps = steps + 1;
        if pos == end {
            break;
        }
    }
    return Some(steps);
}

fn part2(directions: &Vec<Direction>, nodes: &HashMap<NodeID, Node>) -> u64 {
    // start positions ending in A
    let mut pos: Vec<NodeID> = nodes.keys().filter(|k| k[2] == b'A').map(|k| *k).collect();
    let cursors = pos.len();
    println!("{} cursors", cursors);
    let mut steps = 0;
    let mut ends = vec![0u64; cursors];
    for dir in iter::repeat(directions).flat_map(|f| f) {
        // advance
        steps = steps + 1;
        for (idx, p) in pos.iter_mut().enumerate() {
            *p = dir.from(&nodes[p]);
            // reached end?
            if p[2] == b'Z' && ends[idx] == 0 {
                ends[idx] = steps;
                println!("{} reached end in {} steps", idx, steps);
                // all ends found?
                if ends.iter().all(|&x| x > 0u64) {
                    return ends.iter().fold(ends[0], |lcm, next| lcm.lcm(next));
                }
            }
        }
    }
    panic!("not found");
}

fn main() {
    let (directions, nodes) = read_input();

    if let Some(steps) = part1(&directions, &nodes) {
        println!("Part 1: reached end in {} steps", steps);
    }
    let steps2 = part2(&directions, &nodes);
    println!("Part 2: reached ends in {} steps", steps2);
}
