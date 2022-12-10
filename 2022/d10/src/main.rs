use std::{
    fmt::Display,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX { value: i32 },
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_words = s.split(' ').count();
        let mut words = s.split(' ');
        match (words.next(), num_words) {
            (Some(word), 1) if word.eq("noop") => Ok(Instruction::Noop),
            (Some(word), 2) if word.eq("addx") => match words.next().unwrap().parse::<i32>() {
                Ok(value) => Ok(Instruction::AddX { value }),
                Err(or) => Err(format!("Invalid argument: {}", or.to_string())),
            },
            _ => Err(format!("Parse error: {}", s)),
        }
    }
}

#[derive(Debug)]
struct CPU<'a> {
    program: &'a [Instruction],
    cycle: usize,
    instr_cycle: usize, // cycles into current instruction
    pc: usize,
    x: i32,
}

impl<'a> CPU<'a> {
    fn new(program: &[Instruction]) -> CPU {
        CPU {
            program,
            cycle: 1,
            instr_cycle: 0,
            pc: 0,
            x: 1,
        }
    }

    fn step(&mut self) {
        assert!(self.pc < self.program.len());
        match self.program[self.pc] {
            Instruction::Noop => self.next_instruction(),
            Instruction::AddX { value } => {
                self.instr_cycle += 1;
                if self.instr_cycle == 2 {
                    self.x += value;
                    self.next_instruction();
                }
            }
        }
        self.cycle += 1;
    }

    fn next_instruction(&mut self) {
        self.pc += 1;
        self.instr_cycle = 0;
    }

    fn is_done(&self) -> bool {
        self.pc == self.program.len()
    }
}

impl Display for CPU<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "c{}: PC={}.{} X={}",
            self.cycle, self.pc, self.instr_cycle, self.x
        )
    }
}

fn read_input() -> Vec<Instruction> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|ln| Instruction::from_str(ln.as_str()).unwrap())
        .collect()
}

fn main() {
    let program = read_input();
    let mut cpu = CPU::new(program.as_slice());
    let mut signal_strength = 0;
    let smol = program.len() < 10;

    if smol {
        println!("{}", cpu);
    }

    while !cpu.is_done() {
        let cycle = cpu.cycle;

        // part 2
        if !smol {
            let sprite = cpu.x - 1..=cpu.x + 1;
            let position = (cycle - 1) as i32 % 40;
            print!("{}", if sprite.contains(&position) { '#' } else { '.' });
            if cycle % 40 == 0 {
                println!("");
            }
        }

        cpu.step();

        if smol {
            println!("{}", cpu);
        }

        // part 1
        if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
            signal_strength += cycle * cpu.x as usize;
        }
    }
    println!("part1: {}", signal_strength);
}
