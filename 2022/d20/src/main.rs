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

fn move_cursor(cursor: &mut CursorMut<(i64, bool)>, moves: i64) {
    if moves > 0 {
        for _ in 0..moves {
            cursor.move_next();
            if cursor.current().is_none() {
                cursor.move_next();
            }
        }
    } else if moves < 0 {
        for _ in 0..-moves {
            cursor.move_prev();
            if cursor.current().is_none() {
                cursor.move_prev();
            }
        }
    }
    print_list("move: ", cursor.as_cursor());
}

fn print_list(_prefix: &str, mut _cursor: Cursor<(i64, bool)>) {}

fn print_list_0(prefix: &str, mut cursor: Cursor<(i64, bool)>) {
    let mut idx = 0;
    loop {
        if cursor.current().is_none() {
            break;
        }
        cursor.move_prev();
        idx += 1;
    }
    cursor.move_next();
    print!("{}", prefix);
    while let Some(&(value, _moved)) = cursor.current() {
        idx -= 1;
        if idx == 0 {
            print!("➡️ ");
        }
        print!("{}, ", value);
        cursor.move_next();
    }
    print!("\n");
}

fn mix(values: &Vec<i64>) -> Vec<i64> {
    let mut ll = values
        .iter()
        .map(|i| (*i, false))
        .collect::<LinkedList<_>>();

    let mut c = ll.cursor_front_mut();
    let max = values.len() as i64 - 1;
    let mut num_moved = 0;
    loop {
        if let Some(&(value, false)) = c.as_cursor().current() {
            let moves = if value >= 0 {
                value % max
            } else {
                max - (value.abs() % max)
            };
            print_list(format!("moving {}:\n", value).as_str(), c.as_cursor());

            num_moved += 1;
            c.remove_current();
            if moves == 0 {
                c.insert_before((value, true));
                c.move_next();
                continue;
            }
            print_list("removed: ", c.as_cursor());

            // move to insertion point
            move_cursor(&mut c, moves);

            // insert
            /*if c.peek_prev().is_none() {
                // insert at end
                c.move_prev();
                c.insert_before((value, true));
                c.move_next();
            } else*/
            {
                // insert here
                c.insert_before((value, true));
                c.move_prev();
            }

            print_list("insert: ", c.as_cursor());

            // move back
            move_cursor(&mut c, -moves);
        } else if num_moved == values.len() {
            break;
        } else {
            c.move_next();
            continue;
        }
    }

    ll.iter().map(|(value, _moved)| *value).collect()
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
    let mixed = mix(values);
    println!("Part1 groove coords: {}", groove_coordinates(&mixed));
}

fn part2(values: &Vec<i64>) {
    panic!("oh fuck")
}

fn main() {
    let input = read_input();

    part1(&input);
    part2(&input);
}
