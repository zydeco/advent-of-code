use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use std::{
    mem,
    ops::{Mul, Rem, Shl},
};

struct Behaviour {
    operation: fn(&BigUint) -> BigUint,
    divisor: u32,
    dst_true: usize,
    dst_false: usize,
}

struct Monkey {
    items: Vec<BigUint>,
    items_inspected: usize,
    behaviour: Behaviour,
}

impl Monkey {
    fn new(
        items: &[i32],
        operation: fn(&BigUint) -> BigUint,
        divisor: u32,
        dst_true: usize,
        dst_false: usize,
    ) -> Monkey {
        Monkey {
            items: items
                .iter()
                .map(i32::to_biguint)
                .map(Option::unwrap)
                .collect(),
            items_inspected: 0,
            behaviour: Behaviour {
                operation,
                divisor,
                dst_true,
                dst_false,
            },
        }
    }
}

fn play_round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        let items = mem::take(&mut monkeys[i].items);
        monkeys[i].items_inspected += items.len();
        let mb = &monkeys[i].behaviour;
        for item in items {
            let new_item = (mb.operation)(&item);
            let dst = if new_item.clone().rem(mb.divisor).is_zero() {
                mb.dst_true
            } else {
                mb.dst_false
            };
            monkeys[dst].items.push(new_item);
        }
    }
}

fn play_rounds(rounds: usize, monkeys: &mut [Monkey]) {
    for r in 0..rounds {
        play_round(monkeys);
        if r == 0 || r == 19 || (r > 20 && r % 1000 == 999) {
            println!("After round {}", r + 1);
            print_inspected(monkeys);
        }
    }
    if rounds % 1000 != 0 && rounds != 1 && rounds != 20 {
        println!("After round {}", rounds);
        print_inspected(monkeys);
    }
}

fn print_state(monkeys: &[Monkey]) {
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", idx, monkey.items);
    }
}

fn print_inspected(monkeys: &[Monkey]) {
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times.",
            idx, monkey.items_inspected
        );
    }
}

fn business_level(monkeys: &[Monkey]) -> usize {
    let mut inspected = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();
    inspected.sort();
    inspected.reverse();
    inspected[0] * inspected[1]
}

macro_rules! pow2mod {
    ($b:expr) => {{
        |x| x.modpow(&BigUint::from(2u32), &BigUint::from($b))
    }};
}

fn main() {
    // test
    let mut _monkeys0 = [
        Monkey::new(&[79, 98], |old| old * 19u8, 23, 2, 3),
        Monkey::new(&[54, 65, 75, 74], |old| old + 6u8, 19, 2, 0),
        Monkey::new(&[79, 60, 97], pow2mod!(96577u32), 13, 1, 3),
        Monkey::new(&[74], |old| old + 3u8, 17, 0, 1),
    ];

    // input
    let mut _monkeys1 = [
        Monkey::new(&[72, 97], |old| old * 13u8, 19, 5, 6),
        Monkey::new(&[55, 70, 90, 74, 95], pow2mod!(9699690u32), 7, 5, 0),
        Monkey::new(&[74, 97, 66, 57], |old| old + 6u8, 17, 1, 0),
        Monkey::new(&[86, 54, 53], |old| old + 2u8, 13, 1, 2),
        Monkey::new(&[50, 65, 78, 50, 62, 99], |old| old + 3u8, 11, 3, 7),
        Monkey::new(&[90], |old| old + 4u8, 2, 4, 6),
        Monkey::new(&[88, 92, 63, 94, 96, 82, 53, 53], |old| old + 8u8, 5, 4, 7),
        Monkey::new(&[70, 60, 71, 69, 77, 70, 98], |old| old * 7u8, 3, 2, 3),
    ];

    let monkeys = &mut _monkeys1;

    play_rounds(10000, monkeys);
    println!("Busines level: {}", business_level(monkeys));
}
