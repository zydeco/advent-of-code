use std::mem;

struct Behaviour {
    operation: fn(i32) -> i32,
    divisor: i32,
    dst_true: usize,
    dst_false: usize,
}

struct Monkey {
    items: Vec<i32>,
    items_inspected: usize,
    behaviour: Behaviour,
}

impl Monkey {
    fn new(
        items: &[i32],
        operation: fn(i32) -> i32,
        divisor: i32,
        dst_true: usize,
        dst_false: usize,
    ) -> Monkey {
        Monkey {
            items: items.into(),
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
        println!("Monkey {}:", i);
        let items = mem::take(&mut monkeys[i].items);
        monkeys[i].items_inspected += items.len();
        let mb = &monkeys[i].behaviour;
        for mut item in items {
            println!("  Monkey inspects an item with a worry level of {}", item);
            println!("    Worry level is transformed to {}", (mb.operation)(item));
            item = (mb.operation)(item) / 3;
            println!(
                "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                item
            );
            let dst = if item.rem_euclid(mb.divisor) == 0 {
                println!("    Current worry level is divisible by {}.", mb.divisor);
                mb.dst_true
            } else {
                println!(
                    "    Current worry level is not divisible by {}.",
                    mb.divisor
                );
                mb.dst_false
            };
            println!(
                "    Item with worry level {} is thrown to monkey {}.",
                item, dst
            );
            monkeys[dst].items.push(item);
        }
    }
}

fn play_rounds(rounds: usize, monkeys: &mut [Monkey]) {
    for r in 0..rounds {
        play_round(monkeys);
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            r + 1
        );
        print_state(monkeys);
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

fn main() {
    // test
    let mut monkeys0 = [
        Monkey::new(&[79, 98], |old| old * 19, 23, 2, 3),
        Monkey::new(&[54, 65, 75, 74], |old| old + 6, 19, 2, 0),
        Monkey::new(&[79, 60, 97], |old| old * old, 13, 1, 3),
        Monkey::new(&[74], |old| old + 3, 17, 0, 1),
    ];

    // input
    let mut monkeys1 = [
        Monkey::new(&[72, 97], |old| old * 13, 19, 5, 6),
        Monkey::new(&[55, 70, 90, 74, 95], |old| old * old, 7, 5, 0),
        Monkey::new(&[74, 97, 66, 57], |old| old + 6, 17, 1, 0),
        Monkey::new(&[86, 54, 53], |old| old + 2, 13, 1, 2),
        Monkey::new(&[50, 65, 78, 50, 62, 99], |old| old + 3, 11, 3, 7),
        Monkey::new(&[90], |old| old + 4, 2, 4, 6),
        Monkey::new(&[88, 92, 63, 94, 96, 82, 53, 53], |old| old + 8, 5, 4, 7),
        Monkey::new(&[70, 60, 71, 69, 77, 70, 98], |old| old * 7, 3, 2, 3),
    ];

    let monkeys = &mut monkeys1;

    play_rounds(20, monkeys);
    print_inspected(monkeys);
    println!("Part1 busines level: {}", business_level(monkeys));
}
