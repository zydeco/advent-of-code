use std::fmt::Display;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Register {
    X, Y, Z, W
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(Register),
    Add(Register, Argument),
    Mul(Register, Argument),
    Div(Register, Argument),
    Mod(Register, Argument),
    Eql(Register, Argument)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Argument {
    Imm(i32),
    Reg(Register)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Default for State {
    fn default() -> State {
        State{x:0, y:0, z:0, w:0}
    }
}

struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
struct ParseError {
    line: usize,
    message: String,
}

impl FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            "w" => Ok(Register::W),
            _ => Err(ParseError{line: 0, message: format!("Unknown register '{}'", s)})
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Register::X => 'x',
            Register::Y => 'y',
            Register::Z => 'z',
            Register::W => 'w'
        })?;
        Ok(())
    }
}

impl FromStr for Argument {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(reg) = Register::from_str(s) {
            return Ok(Argument::Reg(reg));
        } else if let Ok(value) = i32::from_str_radix(s, 10) {
            return Ok(Argument::Imm(value))
        } else {
            return Err(ParseError{line: 0, message: format!("Invalid argument '{}'", s)})
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Argument::Imm(value) => write!(f, "{}", value)?,
            Argument::Reg(reg) => write!(f, "{}", reg)?,
        }
        Ok(())
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_ascii_whitespace().collect::<Vec<_>>();

        // parse arguments
        let mnemonic = words[0];
        let mut args = vec![];
        for word in words[1..].iter() {
            match Argument::from_str(word) {
                Err(err) => return Err(err),
                Ok(arg) => args.push(arg)
            }
        }

        // parse instruction
        if mnemonic == "inp" {
            // 1 register argument
            if args.len() != 1 {
                return Err(ParseError{line:0, message: format!("Expected 1 argument, got {}", args.len())})
            } else if let Argument::Reg(reg) = args[0] {
                return Ok(Instruction::Inp(reg))
            } else {
                return Err(ParseError{line:0, message: format!("Expected first argument to be register got {}", args[0])})
            }
        } else if args.len() != 2 {
            return Err(ParseError{line:0, message: format!("Expected 2 arguments, got {}", args.len())})
        } else if let Argument::Reg(arg1) = args[0] {
            let arg2 = args[1];
            match mnemonic {
                "add" => return Ok(Instruction::Add(arg1, arg2)),
                "mul" => return Ok(Instruction::Mul(arg1, arg2)),
                "div" => return Ok(Instruction::Div(arg1, arg2)),
                "mod" => return Ok(Instruction::Mod(arg1, arg2)),
                "eql" => return Ok(Instruction::Eql(arg1, arg2)),
                _ => return Err(ParseError{line:0, message: format!("Unknown instruction {}", mnemonic)})
            }
        } else {
            return Err(ParseError{line:0, message: format!("Expected first argument to be register got {}", args[0])})
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Instruction::Inp(reg) => write!(f, "inp {}", reg)?,
            Instruction::Add(reg, arg) => write!(f, "add {} {}", reg, arg)?,
            Instruction::Mul(reg, arg) => write!(f, "mul {} {}", reg, arg)?,
            Instruction::Div(reg, arg) => write!(f, "div {} {}", reg, arg)?,
            Instruction::Mod(reg, arg) => write!(f, "mod {} {}", reg, arg)?,
            Instruction::Eql(reg, arg) => write!(f, "eql {} {}", reg, arg)?,
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ExecutionError {
    EndOfInput,
    DivideByZero,
    InvalidMod,
}

impl State {
    fn get(&self, register: &Register) -> i32 {
        match register {
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::W => self.w,
        }
    }

    fn set(&mut self, register: &Register, value: i32) {
        match register {
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
            Register::W => self.w = value,
        }
    }

    fn stack(&self) -> Vec<u8> {
        let mut stack = vec![];
        let mut z = self.z;
        while z > 0 {
            stack.insert(0, (z % 26) as u8);
            z = z / 26;
        }
        stack
    }
}

impl Instruction {
    fn register(&self) -> Register {
        match self {
            Instruction::Inp(reg) => *reg,
            Instruction::Add(reg, _) => *reg,
            Instruction::Mul(reg, _) => *reg,
            Instruction::Div(reg, _) => *reg,
            Instruction::Mod(reg, _) => *reg,
            Instruction::Eql(reg, _) => *reg,
        }
    }

    fn execute<'a, I>(&self, prev_state: &State, input: &mut I) -> Result<State, ExecutionError>
    where I: Iterator<Item = &'a i32> {
        let mut state = *prev_state;
        let get = |state: &State, arg: &Argument| -> i32 {
            match arg {
                Argument::Imm(ediate) => *ediate,
                Argument::Reg(ister) => state.get(ister)
            }
        };
        match self {
            Instruction::Inp(reg) => {
                if let Some(value) = input.next() {
                    state.set(reg, *value);
                } else {
                    return Err(ExecutionError::EndOfInput)
                }
            },
            Instruction::Add(reg, arg) => {
                state.set(reg, state.get(reg) + get(&state, arg));
            },
            Instruction::Mul(reg, arg) => {
                state.set(reg, state.get(reg) * get(&state, arg));
            },
            Instruction::Div(reg, arg) => {
                let dividend = get(&state, arg);
                if dividend == 0 {
                    return Err(ExecutionError::DivideByZero)
                }
                state.set(reg, state.get(reg) / get(&state, arg));
            },
            Instruction::Mod(reg, arg) => {
                let a = state.get(reg);
                let b = get(&state, arg);
                if a < 0 || b <= 0 {
                    return Err(ExecutionError::InvalidMod)
                }
                state.set(reg, state.get(reg) % get(&state, arg));
            },
            Instruction::Eql(reg, arg) => {
                state.set(reg, if state.get(reg) == get(&state, arg) { 1 } else { 0 });
            },
        }
        if self.register() == Register::Z && prev_state.z != state.z && state.z % 26 != 0 {
            println!("state: {}, stack={:?}", state, state.stack());
        }
        Ok(state)
    }
}

impl Program {
    fn run(&self, input: &[i32]) -> Result<State, ExecutionError> {
        let mut input = input.iter();
        self.instructions.iter().try_fold(State::default(), |state, &i| i.execute(&state, &mut input))
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for i in self.instructions.iter() {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "x={},y={},z={},w={}", self.x, self.y, self.z, self.w)?;
        Ok(())
    }
}

fn read_input() -> Program {
    Program{
        instructions: io::stdin().lock().lines()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|line| !line.starts_with("#") )
            .map(|line| Instruction::from_str(&line).ok().unwrap() )
            .collect::<Vec<_>>()
    }
}

fn fourteen_digits(i: u64) -> [i32; 14] {
    [
        ((i / 10000000000000) % 10) as i32,
        ((i / 1000000000000) % 10) as i32,
        ((i / 100000000000) % 10) as i32,
        ((i / 10000000000) % 10) as i32,
        ((i / 1000000000) % 10) as i32,
        ((i / 100000000) % 10) as i32,
        ((i / 10000000) % 10) as i32,
        ((i / 1000000) % 10) as i32,
        ((i / 100000) % 10) as i32,
        ((i / 10000) % 10) as i32,
        ((i / 1000) % 10) as i32,
        ((i / 100) % 10) as i32,
        ((i / 10) % 10) as i32,
        (i % 10) as i32,
    ]
}

fn main() {
    let program = read_input();
    let model = 79997391969649;
    let input = fourteen_digits(model);
    let result = program.run(&input);

    println!("result: {}", result.unwrap());
}
