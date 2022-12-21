use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

type Name = [u8; 4];

#[derive(Debug)]
enum Value {
    Number(i64),
    Add(Name, Name),
    Subtract(Name, Name),
    Multiply(Name, Name),
    Divide(Name, Name),
    Compare(Name, Name),
}

fn read_name(buf: &[u8]) -> Name {
    buf.try_into().unwrap()
}

fn parse_value(line: &str) -> (Name, Value) {
    assert!(line.len() > 6);
    let name = read_name(line[0..4].as_bytes());
    assert_eq!(line.chars().nth(4), Some(':'));
    assert_eq!(line.chars().nth(5), Some(' '));
    let value_str = &line[6..];
    if value_str.len() == 11 {
        let lhs = read_name(value_str[0..4].as_bytes());
        let op = value_str.chars().nth(5).unwrap();
        let rhs = read_name(value_str[7..11].as_bytes());
        return (
            name,
            match op {
                '+' => Value::Add(lhs, rhs),
                '-' => Value::Subtract(lhs, rhs),
                '/' => Value::Divide(lhs, rhs),
                '*' => Value::Multiply(lhs, rhs),
                _ => panic!("Unknown operation '{}'", value_str),
            },
        );
    }
    return (name, Value::Number(value_str.parse().unwrap()));
}

fn read_input() -> HashMap<Name, Value> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|ln| !ln.is_empty())
        .map(|ln| parse_value(&ln))
        .collect()
}

macro_rules! name {
    ($i: ident) => {{
        let name: Name = stringify!($i).as_bytes().try_into().unwrap();
        name
    }};
}

fn name_to_string(name: &Name) -> String {
    String::from_iter(name.iter().map(|i| *i as char))
}

impl Value {
    fn resolve(&self, values: &HashMap<Name, Value>) -> i64 {
        if let Value::Number(n) = self {
            return *n;
        }
        let (lhs, rhs) = match self {
            Value::Add(lhs, rhs)
            | Value::Subtract(lhs, rhs)
            | Value::Multiply(lhs, rhs)
            | Value::Divide(lhs, rhs) => (
                values.get(lhs).unwrap().resolve(values),
                values.get(rhs).unwrap().resolve(values),
            ),
            _ => panic!("Unresolvable operation!"),
        };

        match self {
            Value::Add(_, _) => lhs + rhs,
            Value::Subtract(_, _) => lhs - rhs,
            Value::Multiply(_, _) => lhs * rhs,
            Value::Divide(_, _) => lhs / rhs,
            _ => unreachable!(),
        }
    }

    fn can_resolve(&self, values: &HashMap<Name, Value>, unresolved: &HashSet<Name>) -> bool {
        match self {
            Value::Number(_) => return true,
            Value::Compare(_, _) => return false,
            _ => (),
        }
        let (lhs, rhs) = self.sides().unwrap();
        if unresolved.contains(&lhs) || unresolved.contains(&rhs) {
            return false;
        }
        let left = values.get(&lhs).unwrap();
        let right = values.get(&rhs).unwrap();
        left.can_resolve(values, unresolved) && right.can_resolve(values, unresolved)
    }

    fn expr(&self, values: &HashMap<Name, Value>, unresolved: &HashSet<Name>) -> String {
        if let Value::Number(n) = self {
            return n.to_string();
        }
        if self.can_resolve(values, unresolved) {
            return self.resolve(values).to_string();
        }

        let (lhs, rhs) = self.sides().unwrap();
        let (lhx, rhx) = (
            if unresolved.contains(&lhs) {
                name_to_string(&lhs)
            } else {
                values.get(&lhs).unwrap().expr(values, unresolved)
            },
            if unresolved.contains(&rhs) {
                name_to_string(&rhs)
            } else {
                values.get(&rhs).unwrap().expr(values, unresolved)
            },
        );
        format!("({} {} {})", self.op_str(), lhx, rhx)
    }

    fn sides(&self) -> Option<(Name, Name)> {
        match self {
            Value::Number(_) => None,
            Value::Add(lhs, rhs)
            | Value::Subtract(lhs, rhs)
            | Value::Multiply(lhs, rhs)
            | Value::Divide(lhs, rhs)
            | Value::Compare(lhs, rhs) => Some((*lhs, *rhs)),
        }
    }

    fn op_str(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Add(_, _) => "+".into(),
            Value::Subtract(_, _) => "-".into(),
            Value::Multiply(_, _) => "*".into(),
            Value::Divide(_, _) => "/".into(),
            Value::Compare(_, _) => "=".into(),
        }
    }
}

fn part1(input: &HashMap<Name, Value>) {
    let root = input.get(&name!(root)).unwrap();
    println!("root says {}", root.resolve(&input));
}

fn part2(input: &HashMap<Name, Value>) {
    let root_sides = input.get(&name!(root)).unwrap().sides().unwrap();
    let cmp_root = Value::Compare(root_sides.0, root_sides.1);
    let unresolved = HashSet::from([name!(humn)]);
    println!("; use z3 to solve it\n(declare-fun humn () Int)\n(assert (> humn 0))\n(assert {})\n(check-sat)\n(get-model)", cmp_root.expr(&input, &unresolved));
}

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}
