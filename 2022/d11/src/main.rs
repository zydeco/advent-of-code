use std::{
    io::{self, Read},
    mem,
    str::FromStr,
};

#[derive(Debug)]
enum Operation {
    Add(u32),
    Mul(u32),
    Pow2(),
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Operation::Add(x) => value + (*x as u64),
            Operation::Mul(x) => value * (*x as u64),
            Operation::Pow2() => value * value,
        }
    }
}

#[derive(Debug)]
struct Behaviour {
    initial_items: Vec<u32>,
    operation: Operation,
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
    fn monkey(&self) -> Monkey {
        Monkey {
            items: self.initial_items.clone(),
            items_inspected: 0,
            behaviour: self,
        }
    }
}

impl FromStr for Behaviour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        assert!(lines.next().unwrap().starts_with("Monkey "));

        let line1 = lines.next().unwrap();
        assert!(line1.starts_with("  Starting items: "));
        let initial_items = line1[18..]
            .split(|c| c == ' ' || c == ',')
            .filter(|w| !w.is_empty())
            .map(|w| w.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let operation = Operation::from_str(lines.next().unwrap()).unwrap();

        let line3 = lines.next().unwrap();
        assert!(line3.starts_with("  Test: divisible by "));
        let divisor = line3.split(' ').last().unwrap().parse::<u32>().unwrap();

        let line4 = lines.next().unwrap();
        assert!(line4.starts_with("    If true: throw to monkey "));
        let dst_true = line4.split(' ').last().unwrap().parse::<usize>().unwrap();

        let line5 = lines.next().unwrap();
        assert!(line5.starts_with("    If false: throw to monkey "));
        let dst_false = line5.split(' ').last().unwrap().parse::<usize>().unwrap();

        Ok(Behaviour {
            initial_items,
            operation,
            divisor,
            dst_true,
            dst_false,
        })
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.starts_with("  Operation: new = old "));
        let mut words = s[23..].split(' ');
        let op = {
            let op_word = words.next().unwrap();
            assert!(op_word.eq("*") || op_word.eq("+"));
            op_word.chars().next().unwrap()
        };
        let operand = words.next().unwrap();
        let operand_parsed = operand.parse::<u32>();
        assert!(operand.eq("old") || operand_parsed.is_ok());
        match (op, operand_parsed.ok()) {
            ('+', Some(value)) => Ok(Operation::Add(value)),
            ('*', Some(value)) => Ok(Operation::Mul(value)),
            ('*', None) => Ok(Operation::Pow2()),
            _ => Err(format!("Invalid operation line: {}", s)),
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
        let mb = monkeys[i].behaviour;
        for item in items {
            let new_item = match mode {
                GameMode::Long { common_divisor } => {
                    (mb.operation.apply(item as u64) % common_divisor) as u32
                }
                GameMode::Short => (mb.operation.apply(item as u64) / 3) as u32,
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

fn read_input() -> Vec<Behaviour> {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Error reading input");
    buf.split("\n\n")
        .map(Behaviour::from_str)
        .map(Result::unwrap)
        .collect()
}

fn main() {
    let input = read_input();

    let part1 = play_rounds(20, input.as_slice());
    println!("---");
    let part2 = play_rounds(10000, input.as_slice());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
