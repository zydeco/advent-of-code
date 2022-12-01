use std::collections::HashMap;

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Player {
    pos: u8,
    score: u8
}

impl Player {
    fn new(pos: u8) -> Player {
        Player{pos: pos, score: 0}
    }

    fn roll(&self, roll: u8) -> Player {
        let mut pos = self.pos + roll;
        while pos > 10 {
            pos -= 10
        }
        Player{pos: pos, score: self.score+pos}
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"[pos:{},score:{}]", self.pos, self.score)?;
        Ok(())
    }
}

fn parse_start_position(s: &String) -> u8 {
    let pos: u8 = s.parse().unwrap_or(0);
    if pos < 1 || pos > 10 {
        eprintln!("Invalid start position {}, expected 1 to 10", s);
        std::process::exit(1);
    }
    pos
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        eprintln!("Usage: {} start-player1 start-player2", args[0]);
        std::process::exit(1);
    }

    let players = args.iter()
        .skip(1)
        .map(parse_start_position)
        .map(Player::new)
        .collect::<Vec<_>>();
    
    if players.len() != 2 {
        eprintln!("Not enough players!");
        std::process::exit(1);
    }

    for (i,p) in players.iter().enumerate() {
        println!("Player {} starts at {}", i+1, p.pos);
    }

    let rolls: Vec<(u64,u8)> = vec![
        // (occurrences, roll)
        (1,3), // 111
        (3,4), // 112 121 211
        (6,5), // 122 212 221 113 131 311
        (7,6), // 222 123 132 213 231 312 321
        (6,7), // 331 313 133 322 232 223
        (3,8), // 332 323 233
        (1,9), // 333
    ];

    let mut cache = HashMap::new();
    let wins = play(&players[0], &players[1], 21, &rolls, &mut cache);
    println!("Player 1 wins in {} universes", wins.0);
    println!("Player 2 wins in {} universes", wins.1);
}

fn play(player1: &Player, player2: &Player, winning_score: u8, rolls: &Vec<(u64,u8)>, cache: &mut HashMap<[Player;2],(u64,u64)>) -> (u64, u64) {
    let (mut wins1, mut wins2) = (0, 0);
    let key = [*player1, *player2];
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }
    for (times1, roll1) in rolls {
        let player1 = player1.roll(*roll1);
        if player1.score >= winning_score {
            wins1 += times1;
            continue;
        }
        for (times2, roll2) in rolls {
            let player2 = player2.roll(*roll2);
            if player2.score >= winning_score {
                wins2 += times1 * times2;
            } else {
                let sub_wins = play(&player1, &player2, winning_score, rolls, cache);
                wins1 += times1 * times2 * sub_wins.0;
                wins2 += times1 * times2 * sub_wins.1;
            }
        }
    }
    cache.insert(key, (wins1, wins2));
    (wins1, wins2)
}
