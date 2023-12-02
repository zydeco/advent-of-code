use std::{
    fmt::Display,
    io::{self, BufRead},
    slice::Iter,
};

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Cubes {
    fn from_str(s: &str) -> Cubes {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for s1 in s.split(", ") {
            let count = i32::from_str_radix(s1.split(' ').next().unwrap(), 10).unwrap();
            if s1.ends_with(" red") {
                red = count;
            } else if s1.ends_with(" blue") {
                blue = count;
            } else if s1.ends_with(" green") {
                green = count;
            } else {
                panic!("aaaahh")
            }
        }
        Cubes { red, green, blue }
    }

    fn max(cubes: &Vec<Cubes>) -> Cubes {
        Cubes {
            red: cubes.iter().map(|r| r.red).max().unwrap(),
            green: cubes.iter().map(|r: &Cubes| r.green).max().unwrap(),
            blue: cubes.iter().map(|r| r.blue).max().unwrap(),
        }
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: i32,
    rounds: Vec<Cubes>,
}

impl Game {
    fn from_string(s: String) -> Game {
        assert!(s.starts_with("Game "));
        let id = i32::from_str_radix(
            s.split(':')
                .next()
                .unwrap()
                .split(' ')
                .skip(1)
                .next()
                .unwrap(),
            10,
        )
        .unwrap();
        let rounds = s
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split("; ")
            .map(Cubes::from_str)
            .collect();
        Game { id, rounds }
    }

    fn is_possible(&self, bag: &Cubes) -> bool {
        for round in &self.rounds {
            if round.red > bag.red || round.green > bag.green || round.blue > bag.blue {
                return false;
            }
        }
        true
    }

    fn needed_cubes(&self) -> Cubes {
        Cubes::max(&self.rounds)
    }
}

impl Display for Cubes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.red > 0 {
            write!(f, "{} red", self.red)?;
            if self.green > 0 || self.blue > 0 {
                write!(f, ", ")?;
            }
        }
        if self.green > 0 {
            write!(f, "{} green", self.green)?;
            if self.blue > 0 {
                write!(f, ", ")?;
            }
        }
        if self.blue > 0 {
            write!(f, "{} blue", self.blue)?;
        }

        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: ", self.id)?;
        for round in self.rounds.iter() {
            write!(f, "{}; ", round)?;
        }
        Ok(())
    }
}

fn main() {
    let games = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(Game::from_string)
        .collect::<Vec<_>>();

    let bag = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_sum = games
        .iter()
        .filter(|&g| g.is_possible(&bag))
        .fold(0, |s, g| s + g.id);

    println!("Possible sum: {}", possible_sum);

    let power_sum: i32 = games.iter().map(|g| g.needed_cubes().power()).sum();
    println!("Power sum: {}", power_sum);
}
