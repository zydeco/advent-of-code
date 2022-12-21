use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use z3::{ast::Ast, ast::Int, Config, Context, Optimize, SatResult};

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

    fn add_constraints(
        &self,
        name: &Name,
        vars: &HashMap<&Name, Int>,
        ctx: &Context,
        opt: &Optimize,
    ) {
        let var = vars.get(name).unwrap();
        if let Value::Number(n) = self {
            opt.assert(&var._eq(&Int::from_i64(ctx, *n)));
        } else if let Some((lhs, rhs)) = self.sides() {
            let left_var = vars.get(&lhs).unwrap();
            let right_var = vars.get(&rhs).unwrap();
            let vars = &[left_var, right_var];
            match self {
                Value::Add(_, _) => opt.assert(&var._eq(&Int::add(&ctx, vars))),
                Value::Subtract(_, _) => opt.assert(&var._eq(&Int::sub(&ctx, vars))),
                Value::Multiply(_, _) => opt.assert(&var._eq(&Int::mul(&ctx, vars))),
                Value::Divide(_, _) => opt.assert(&var._eq(&left_var.div(right_var))),
                Value::Compare(_, _) => opt.assert(&left_var._eq(right_var)),
                _ => unreachable!(),
            }
        }
    }
}

fn part1(input: &HashMap<Name, Value>) {
    let root = input.get(&name!(root)).unwrap();
    println!("root says {}", root.resolve(&input));
}

fn part2(input: &HashMap<Name, Value>) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    // declare variables
    let mut vars = HashMap::new();
    for name in input.keys() {
        let var = Int::new_const(&ctx, name_to_string(name));
        vars.insert(name, var);
    }

    // add constraints for all except root and humn
    let root_name = name!(root);
    let humn_name = name!(humn);
    for (name, value) in input
        .iter()
        .filter(|(&name, _)| name != root_name && name != humn_name)
    {
        value.add_constraints(name, &vars, &ctx, &opt);
    }

    // add constraints for root
    let root_sides = input.get(&name!(root)).unwrap().sides().unwrap();
    let cmp_root = Value::Compare(root_sides.0, root_sides.1);
    cmp_root.add_constraints(&root_name, &vars, &ctx, &opt);

    // solve
    let humn_var = vars.get(&humn_name).unwrap();
    assert_eq!(opt.check(&[]), SatResult::Sat);
    let m = opt.get_model().unwrap();
    println!("human must say {}", m.eval(humn_var, true).unwrap());
}

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}
