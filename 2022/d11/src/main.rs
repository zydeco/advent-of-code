use std::mem;

struct Behaviour {
    initial_items: Vec<u32>,
    operation: fn(u64) -> u64,
    divisor: u32,
    dst_true: usize,
    dst_false: usize,
}

struct Monkey<'a> {
    items: Vec<u32>,
    items_inspected: usize,
    behaviour: &'a Behaviour,
}

impl Behaviour {
    fn new(
        items: &[u32],
        operation: fn(u64) -> u64,
        divisor: u32,
        dst_true: usize,
        dst_false: usize,
    ) -> Behaviour {
        Behaviour {
            initial_items: items.into(),
            operation,
            divisor,
            dst_true,
            dst_false,
        }
    }

    fn monkey(&self) -> Monkey {
        Monkey {
            items: self.initial_items.clone(),
            items_inspected: 0,
            behaviour: self,
        }
    }
}

enum GameMode {
    Short,
    Long { common_divisor: u64 },
}

fn play_round(monkeys: &mut [Monkey], mode: &GameMode) {
    for i in 0..monkeys.len() {
        let items = mem::take(&mut monkeys[i].items);
        monkeys[i].items_inspected += items.len();
        let mb = &monkeys[i].behaviour;
        for item in items {
            let new_item = match mode {
                GameMode::Long { common_divisor } => {
                    ((mb.operation)(item as u64) % common_divisor) as u32
                }
                GameMode::Short => ((mb.operation)(item as u64) / 3) as u32,
            };
            let dst = if new_item % mb.divisor == 0 {
                mb.dst_true
            } else {
                mb.dst_false
            };
            monkeys[dst].items.push(new_item);
        }
    }
}

fn play_rounds(rounds: usize, behaviours: &[Behaviour]) -> usize {
    let mode = if rounds > 1000 {
        let common_divisor = behaviours.iter().fold(1, |acc, b| acc * b.divisor) as u64;
        GameMode::Long { common_divisor }
    } else {
        GameMode::Short
    };

    let mut vec_monkeys = behaviours.iter().map(|b| b.monkey()).collect::<Vec<_>>();
    let mut monkeys = vec_monkeys.as_mut();

    for r in 0..rounds {
        play_round(&mut monkeys, &mode);
        if r == 0 || r == 19 || (r > 20 && r % 1000 == 999) {
            println!("After round {}", r + 1);
            print_inspected(monkeys);
        }
    }
    if rounds % 1000 != 0 && rounds != 1 && rounds != 20 {
        println!("After round {}", rounds);
        print_inspected(monkeys);
    }

    business_level(monkeys)
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

fn play_check(rounds: usize, monkeys: &[Behaviour], expected_level: usize) {
    let level = play_rounds(rounds, monkeys);
    assert_eq!(level, expected_level);
}

fn main() {
    // test
    let monkeys0 = [
        Behaviour::new(&[79, 98], |old| old * 19, 23, 2, 3),
        Behaviour::new(&[54, 65, 75, 74], |old| old + 6, 19, 2, 0),
        Behaviour::new(&[79, 60, 97], |old| old * old, 13, 1, 3),
        Behaviour::new(&[74], |old| old + 3, 17, 0, 1),
    ];

    // input
    let monkeys1 = [
        Behaviour::new(&[72, 97], |old| old * 13, 19, 5, 6),
        Behaviour::new(&[55, 70, 90, 74, 95], |old| old * old, 7, 5, 0),
        Behaviour::new(&[74, 97, 66, 57], |old| old + 6, 17, 1, 0),
        Behaviour::new(&[86, 54, 53], |old| old + 2, 13, 1, 2),
        Behaviour::new(&[50, 65, 78, 50, 62, 99], |old| old + 3, 11, 3, 7),
        Behaviour::new(&[90], |old| old + 4, 2, 4, 6),
        Behaviour::new(&[88, 92, 63, 94, 96, 82, 53, 53], |old| old + 8, 5, 4, 7),
        Behaviour::new(&[70, 60, 71, 69, 77, 70, 98], |old| old * 7, 3, 2, 3),
    ];

    play_check(20, &monkeys0, 10605);
    play_check(20, &monkeys1, 58056);
    play_check(10000, &monkeys0, 2713310158);
    play_check(10000, &monkeys1, 15048718170);
    println!("all ok!");
}
