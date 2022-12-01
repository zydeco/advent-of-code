pub enum Direction {
    Forward,
    Up,
    Down
}

pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32
}

pub struct Command {
    pub direction: Direction,
    pub value: i32
}

impl std::ops::Add<&Command> for Position {
    type Output = Position;

    fn add(self, cmd: &Command) -> Position {
        match cmd.direction {
            Direction::Forward => Position {horizontal: self.horizontal + cmd.value, depth: self.depth + (self.aim * cmd.value), aim: self.aim},
            Direction::Up => Position {horizontal: self.horizontal, depth: self.depth, aim: self.aim - cmd.value},
            Direction::Down => Position {horizontal: self.horizontal, depth: self.depth, aim: self.aim + cmd.value}
        }
    }
}
