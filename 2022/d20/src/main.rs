#![feature(linked_list_cursors)]

use std::{
    collections::{
        linked_list::{Cursor, CursorMut},
        HashSet, LinkedList,
    },
    io::{self, BufRead},
};

// input might contain duplicates
fn read_input() -> Vec<i64> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|ln| ln.parse().unwrap())
        .collect()
}

fn move_cursor(cursor: &mut CursorMut<(i64, usize)>, moves: i64) {
    assert!(moves >= 0);
    if cursor.current().is_none() {
        cursor.move_next();
    }
    for _ in 0..moves {
        cursor.move_next();
        if cursor.current().is_none() {
            cursor.move_next();
        }
    }
}

fn move_to_index(cursor: &mut CursorMut<(i64, usize)>, index: usize) -> i64 {
    loop {
        match cursor.current() {
            Some((value, i)) if *i == index => return *value,
            Some(_) | None => cursor.move_next(),
        }
    }
}

fn mix(values: &Vec<i64>, times: usize) -> Vec<i64> {
    let mut ll = values
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, index))
        .collect::<LinkedList<_>>();

    let mut cursor = ll.cursor_front_mut();
    let max = values.len() as i64 - 1;

    for _ in 0..times {
        for index in 0..=(max as usize) {
            let value = move_to_index(&mut cursor, index);
            let moves = value.rem_euclid(max);
            cursor.remove_current();
            move_cursor(&mut cursor, moves);
            cursor.insert_before((value, index));
        }
    }

    ll.iter().map(|(value, _index)| *value).collect()
}

fn get_wrapping(values: &Vec<i64>, index: usize) -> i64 {
    *values.get(index % values.len()).unwrap()
}

fn groove_coordinates(values: &Vec<i64>) -> i64 {
    let zero_index = values.iter().position(|&x| x == 0).unwrap();
    get_wrapping(values, zero_index + 1000)
        + get_wrapping(values, zero_index + 2000)
        + get_wrapping(values, zero_index + 3000)
}

fn part1(values: &Vec<i64>) {
    let mixed = mix(values, 1);
    println!("Part1 groove coords: {}", groove_coordinates(&mixed));
}

fn part2(values: &Vec<i64>) {
    let values2 = values.iter().map(|v| (v * 811589153)).collect();
    let mixed = mix(&values2, 10);
    println!("Part2 groove coords: {}", groove_coordinates(&mixed));
}

fn main() {
    let input = read_input();

    part1(&input);
    part2(&input);
}
