
#[derive(Debug)]
struct Player {
    pos: u32,
    score: u32
}

impl Player {
    fn roll(&mut self, values: &[u8]) {
        let roll = values.iter().fold(0u32, |acc,n| acc+(*n as u32));
        self.pos += roll;
        while self.pos > 10 {
            self.pos -= 10
        }
        self.score += self.pos;
    }
}

#[inline]
fn wrap_101(value: u8) -> u8 {
    if value > 100 { value - 100 } else { value }
}

fn main() {
    let mut players = [
        Player{pos: 8, score: 0},
        Player{pos: 4, score: 0}
    ];
    let mut next_roll = 1u8;
    let mut cur_player = 0;
    let mut dice_rolls = 0u32;
    loop {
        let rolls = [next_roll, wrap_101(next_roll+1), wrap_101(next_roll+2)];
        dice_rolls += 3;
        next_roll = wrap_101(next_roll + 3);
        let player = &mut players[cur_player];
        player.roll(&rolls);
        println!("Player {} rolls {}+{}+{} and moves to space {} for a total score of {}", cur_player+1, rolls[0], rolls[1], rolls[2], player.pos, player.score);
        if player.score >= 1000 {
            println!("Player {} wins after {} dice rolls.", cur_player+1, dice_rolls);
            let other_player_score = players[(cur_player + 1) % players.len()].score;
            println!("Result is {}", other_player_score * dice_rolls);
            dbg!(players);
            break;
        }
        cur_player = (cur_player + 1) % players.len();
    }
}
