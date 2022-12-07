use std::{
    collections::HashSet,
    io::{self, Read},
};

fn is_marker(bytes: &[u8]) -> bool {
    let set: HashSet<u8> = bytes.iter().map(|s| *s).collect();
    set.len() == bytes.len()
}

fn find_marker(bytes: &[u8], marker_size: usize) -> Option<usize> {
    bytes
        .windows(marker_size)
        .position(is_marker)
        .and_then(|x| Some(x + marker_size))
}

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let bytes = buf.as_bytes();
    println!("part1 {}", find_marker(bytes, 4).unwrap());
    println!("part2 {}", find_marker(bytes, 14).unwrap());
}
