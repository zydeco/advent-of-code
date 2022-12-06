use std::{
    collections::HashSet,
    io::{self, Read},
};

fn is_marker(bytes: &[u8]) -> bool {
    let set: HashSet<u8> = bytes.iter().map(|s| *s).collect();
    set.len() == bytes.len()
}

fn find_marker(bytes: &[u8], marker_size: usize) -> Option<usize> {
    for i in 0..bytes.len() - marker_size {
        if is_marker(&bytes[i..i + marker_size]) {
            return Some(i + marker_size);
        }
    }
    None
}

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let bytes = buf.as_bytes();
    println!("part1 {}", find_marker(bytes, 4).unwrap());
    println!("part2 {}", find_marker(bytes, 14).unwrap());
}
