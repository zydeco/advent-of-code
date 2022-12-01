use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::{HashMap,HashSet,BinaryHeap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HallPosition {
    P1, P2, P3, P4, P5, P6, P7
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Kind {
    Amber,
    Bronze,
    Copper,
    Desert
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Side {
    Top,
    Bottom,
    Bottomer,
    Bottomest
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
enum Position {
    Hall(HallPosition),
    Room(Kind, Side)
}


/*
#############
#12.3.4.5.67#
###A#B#C#D###
  #A#C#A#B#
  #A#C#A#B#
  #A#C#A#B#
  #########
*/

const CONNECTIONS: [(Position, Position, u8); 26] = [
    (Position::Hall(HallPosition::P1), Position::Hall(HallPosition::P2), 1),
    (Position::Hall(HallPosition::P2), Position::Hall(HallPosition::P3), 2),
    (Position::Hall(HallPosition::P3), Position::Hall(HallPosition::P4), 2),
    (Position::Hall(HallPosition::P4), Position::Hall(HallPosition::P5), 2),
    (Position::Hall(HallPosition::P5), Position::Hall(HallPosition::P6), 2),
    (Position::Hall(HallPosition::P6), Position::Hall(HallPosition::P7), 1),

    (Position::Hall(HallPosition::P2), Position::Room(Kind::Amber, Side::Top), 2),
    (Position::Hall(HallPosition::P3), Position::Room(Kind::Amber, Side::Top), 2),

    (Position::Hall(HallPosition::P3), Position::Room(Kind::Bronze, Side::Top), 2),
    (Position::Hall(HallPosition::P4), Position::Room(Kind::Bronze, Side::Top), 2),
    
    (Position::Hall(HallPosition::P4), Position::Room(Kind::Copper, Side::Top), 2),
    (Position::Hall(HallPosition::P5), Position::Room(Kind::Copper, Side::Top), 2),
    
    (Position::Hall(HallPosition::P5), Position::Room(Kind::Desert, Side::Top), 2),
    (Position::Hall(HallPosition::P6), Position::Room(Kind::Desert, Side::Top), 2),
    
    (Position::Room(Kind::Amber, Side::Top), Position::Room(Kind::Amber, Side::Bottom), 1),
    (Position::Room(Kind::Bronze, Side::Top), Position::Room(Kind::Bronze, Side::Bottom), 1),
    (Position::Room(Kind::Copper, Side::Top), Position::Room(Kind::Copper, Side::Bottom), 1),
    (Position::Room(Kind::Desert, Side::Top), Position::Room(Kind::Desert, Side::Bottom), 1),
    
    (Position::Room(Kind::Amber,  Side::Bottom), Position::Room(Kind::Amber,  Side::Bottomer), 1),
    (Position::Room(Kind::Bronze, Side::Bottom), Position::Room(Kind::Bronze, Side::Bottomer), 1),
    (Position::Room(Kind::Copper, Side::Bottom), Position::Room(Kind::Copper, Side::Bottomer), 1),
    (Position::Room(Kind::Desert, Side::Bottom), Position::Room(Kind::Desert, Side::Bottomer), 1),
    
    (Position::Room(Kind::Amber,  Side::Bottomer), Position::Room(Kind::Amber,  Side::Bottomest), 1),
    (Position::Room(Kind::Bronze, Side::Bottomer), Position::Room(Kind::Bronze, Side::Bottomest), 1),
    (Position::Room(Kind::Copper, Side::Bottomer), Position::Room(Kind::Copper, Side::Bottomest), 1),
    (Position::Room(Kind::Desert, Side::Bottomer), Position::Room(Kind::Desert, Side::Bottomest), 1),
];

const POSITIONS: [Position; 23] = [
    Position::Hall(HallPosition::P1),
    Position::Hall(HallPosition::P2),
    Position::Hall(HallPosition::P3),
    Position::Hall(HallPosition::P4),
    Position::Hall(HallPosition::P5),
    Position::Hall(HallPosition::P6),
    Position::Hall(HallPosition::P7),
    Position::Room(Kind::Amber, Side::Top),
    Position::Room(Kind::Amber, Side::Bottom),
    Position::Room(Kind::Amber, Side::Bottomer),
    Position::Room(Kind::Amber, Side::Bottomest),
    Position::Room(Kind::Bronze, Side::Top),
    Position::Room(Kind::Bronze, Side::Bottom),
    Position::Room(Kind::Bronze, Side::Bottomer),
    Position::Room(Kind::Bronze, Side::Bottomest),
    Position::Room(Kind::Copper, Side::Top),
    Position::Room(Kind::Copper, Side::Bottom),
    Position::Room(Kind::Copper, Side::Bottomer),
    Position::Room(Kind::Copper, Side::Bottomest),
    Position::Room(Kind::Desert, Side::Top),
    Position::Room(Kind::Desert, Side::Bottom),
    Position::Room(Kind::Desert, Side::Bottomer),
    Position::Room(Kind::Desert, Side::Bottomest),
];

impl Default for Position {
    fn default() -> Self {
        Position::Room(Kind::Amber, Side::Top)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Amphipod {
    kind: Kind,
    index: usize,
}

const AMPHIPODS: [Amphipod; 16] = [
    // index maps to positions in State struct
    Amphipod{kind: Kind::Amber, index: 0},
    Amphipod{kind: Kind::Amber, index: 1},
    Amphipod{kind: Kind::Amber, index: 2},
    Amphipod{kind: Kind::Amber, index: 3},
    Amphipod{kind: Kind::Bronze, index: 4},
    Amphipod{kind: Kind::Bronze, index: 5},
    Amphipod{kind: Kind::Bronze, index: 6},
    Amphipod{kind: Kind::Bronze, index: 7},
    Amphipod{kind: Kind::Copper, index: 8},
    Amphipod{kind: Kind::Copper, index: 9},
    Amphipod{kind: Kind::Copper, index: 10},
    Amphipod{kind: Kind::Copper, index: 11},
    Amphipod{kind: Kind::Desert, index: 12},
    Amphipod{kind: Kind::Desert, index: 13},
    Amphipod{kind: Kind::Desert, index: 14},
    Amphipod{kind: Kind::Desert, index: 15},
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    energy: u32,
    positions: [Position; 16], // A A B B C C D D
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Position::Hall(HallPosition::P1) => "Hall::P1",
            Position::Hall(HallPosition::P2) => "Hall::P2",
            Position::Hall(HallPosition::P3) => "Hall::P3",
            Position::Hall(HallPosition::P4) => "Hall::P4",
            Position::Hall(HallPosition::P5) => "Hall::P5",
            Position::Hall(HallPosition::P6) => "Hall::P6",
            Position::Hall(HallPosition::P7) => "Hall::P7",
            Position::Room(Kind::Amber, Side::Top) => "Room::A::Top",
            Position::Room(Kind::Amber, Side::Bottom) =>  "Room::A::MiddleTop",
            Position::Room(Kind::Amber, Side::Bottomer) =>  "Room::A::MiddleBottom",
            Position::Room(Kind::Amber, Side::Bottomest) =>  "Room::A::Bottom",
            Position::Room(Kind::Bronze, Side::Top) => "Room::B::Top",
            Position::Room(Kind::Bronze, Side::Bottom) => "Room::B::MiddleTop",
            Position::Room(Kind::Bronze, Side::Bottomer) => "Room::B::MiddleBottom",
            Position::Room(Kind::Bronze, Side::Bottomest) => "Room::B::Bottom",
            Position::Room(Kind::Copper, Side::Top) => "Room::C::Top",
            Position::Room(Kind::Copper, Side::Bottom) => "Room::C::MiddleTop",
            Position::Room(Kind::Copper, Side::Bottomer) => "Room::C::MiddleBottom",
            Position::Room(Kind::Copper, Side::Bottomest) => "Room::C::Bottom",
            Position::Room(Kind::Desert, Side::Top) => "Room::D::Top",
            Position::Room(Kind::Desert, Side::Bottom) => "Room::D::MiddleTop",
            Position::Room(Kind::Desert, Side::Bottomer) => "Room::D::MiddleBottom",
            Position::Room(Kind::Desert, Side::Bottomest) => "Room::D::Bottom",
        })?;
        Ok(())
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "#############")?;
        writeln!(f, "#{}{}.{}.{}.{}.{}{}#",
            self.kind_in(Position::Hall(HallPosition::P1)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Hall(HallPosition::P2)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Hall(HallPosition::P3)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Hall(HallPosition::P4)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Hall(HallPosition::P5)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Hall(HallPosition::P6)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Hall(HallPosition::P7)).map(Kind::to_char).unwrap_or('.'),
        )?;
        writeln!(f, "###{}#{}#{}#{}###",
            self.kind_in(Position::Room(Kind::Amber, Side::Top)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Bronze, Side::Top)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Copper, Side::Top)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Desert, Side::Top)).map(Kind::to_char).unwrap_or('.'))?;
        writeln!(f, "  #{}#{}#{}#{}#",
            self.kind_in(Position::Room(Kind::Amber, Side::Bottom)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Bronze, Side::Bottom)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Copper, Side::Bottom)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Desert, Side::Bottom)).map(Kind::to_char).unwrap_or('.'))?;
        writeln!(f, "  #{}#{}#{}#{}#",
            self.kind_in(Position::Room(Kind::Amber,  Side::Bottomer)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Bronze, Side::Bottomer)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Copper, Side::Bottomer)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Desert, Side::Bottomer)).map(Kind::to_char).unwrap_or('.'))?;
        writeln!(f, "  #{}#{}#{}#{}#",
            self.kind_in(Position::Room(Kind::Amber,  Side::Bottomest)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Bronze, Side::Bottomest)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Copper, Side::Bottomest)).map(Kind::to_char).unwrap_or('.'),
            self.kind_in(Position::Room(Kind::Desert, Side::Bottomest)).map(Kind::to_char).unwrap_or('.'))?;
        writeln!(f, "  #########")?;
        Ok(())
    }
}


fn precompute_paths() -> HashMap<(Position,Position),(u32, Vec<Position>)> {
    let mut paths = HashMap::new();
    for p in POSITIONS {
        for q in POSITIONS {
            if p == q {
                continue;
            }
            paths.insert((p,q), p.path(q, &paths));
        }
    }
    paths
}

type PathCache = HashMap<(Position,Position),(u32, Vec<Position>)>;

impl Position {
    fn distance(&self, to: &Position) -> u32 {
        for (a,b,d) in CONNECTIONS {
            if a == *self && b == *to || a == *to && b == *self {
                return d as u32;
            }
        }
        panic!("no distance from {} to {}", self, to);
    }

    fn neighbours(&self) -> Vec<Position> {
        match self {
            Position::Hall(HallPosition::P1) => vec![Position::Hall(HallPosition::P2)],
            Position::Hall(HallPosition::P2) => vec![Position::Hall(HallPosition::P1), Position::Hall(HallPosition::P3), Position::Room(Kind::Amber, Side::Top)],
            Position::Hall(HallPosition::P3) => vec![Position::Hall(HallPosition::P2), Position::Hall(HallPosition::P4), Position::Room(Kind::Amber, Side::Top), Position::Room(Kind::Bronze, Side::Top)],
            Position::Hall(HallPosition::P4) => vec![Position::Hall(HallPosition::P3), Position::Hall(HallPosition::P5), Position::Room(Kind::Bronze, Side::Top), Position::Room(Kind::Copper, Side::Top)],
            Position::Hall(HallPosition::P5) => vec![Position::Hall(HallPosition::P4), Position::Hall(HallPosition::P6), Position::Room(Kind::Copper, Side::Top), Position::Room(Kind::Desert, Side::Top)],
            Position::Hall(HallPosition::P6) => vec![Position::Hall(HallPosition::P5), Position::Hall(HallPosition::P7), Position::Room(Kind::Desert, Side::Top)],
            Position::Hall(HallPosition::P7) => vec![Position::Hall(HallPosition::P6)],
            Position::Room(Kind::Amber, Side::Top) => vec![Position::Hall(HallPosition::P2), Position::Hall(HallPosition::P3), Position::Room(Kind::Amber, Side::Bottom)],
            Position::Room(Kind::Bronze, Side::Top) => vec![Position::Hall(HallPosition::P3), Position::Hall(HallPosition::P4), Position::Room(Kind::Bronze, Side::Bottom)],
            Position::Room(Kind::Copper, Side::Top) => vec![Position::Hall(HallPosition::P4), Position::Hall(HallPosition::P5), Position::Room(Kind::Copper, Side::Bottom)],
            Position::Room(Kind::Desert, Side::Top) => vec![Position::Hall(HallPosition::P5), Position::Hall(HallPosition::P6), Position::Room(Kind::Desert, Side::Bottom)],
            Position::Room(k, Side::Bottom) => vec![Position::Room(*k, Side::Top), Position::Room(*k, Side::Bottomer)],
            Position::Room(k, Side::Bottomer) => vec![Position::Room(*k, Side::Bottom), Position::Room(*k, Side::Bottomest)],
            Position::Room(k, Side::Bottomest) => vec![Position::Room(*k, Side::Bottomer)],
        }
    }

    fn path(&self, to: Position, cache: &PathCache) -> (u32, Vec<Position>) {
        let from = *self;
        if from == to {
            return (0, vec![]);
        }
        if let Some(cached) = cache.get(&(from, to)) {
            return cached.clone();
        }
        let mut queue: HashSet<Position> = HashSet::from_iter(POSITIONS.into_iter());
        let mut dist: HashMap<Position, u32> = HashMap::new();
        let mut prev: HashMap<Position, Position> = HashMap::new();
        dist.insert(from, 0);
        prev.insert(from, from);
        
        while !queue.is_empty() {
            let u = *queue.iter().min_by_key(|pos| dist.get(&pos).unwrap_or(&u32::MAX)).unwrap();
            queue.remove(&u);
            let neighbours = u.neighbours();
            for v in neighbours.iter().filter(|p| queue.contains(p)) {
                let alt = dist.get(&u).unwrap() + u.distance(v);
                if alt < *dist.get(&v).unwrap_or(&u32::MAX) {
                    dist.insert(*v, alt);
                    prev.insert(*v, u);
                }
            }
        }
        
        let mut path = vec![to];
        let mut last = prev.get(&to).unwrap();
        while *last != from {
            path.insert(0, *last);
            last = prev.get(last).unwrap();
        }
        (*dist.get(&to).unwrap(), path)
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

impl Kind {
    fn from_char(c: char) -> Kind {
        match c {
            'A' => Kind::Amber,
            'B' => Kind::Bronze,
            'C' => Kind::Copper,
            'D' => Kind::Desert,
            _ => panic!("No room for {}", c)
        }
    }

    fn to_char(self) -> char {
        match self {
            Kind::Amber => 'A',
            Kind::Bronze => 'B',
            Kind::Copper => 'C',
            Kind::Desert => 'D',
        }
    }

    fn cost_multiplier(self) -> u32 {
        match self {
            Kind::Amber => 1,
            Kind::Bronze => 10,
            Kind::Copper => 100,
            Kind::Desert => 1000,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
            .then_with(|| self.positions.cmp(&other.positions))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn is_final(&self) -> bool {
        use Position::*;
        use Kind::*;
        match self.positions {
            [
                Room(Amber,_),Room(Amber,_),Room(Amber,_),Room(Amber,_),
                Room(Bronze,_),Room(Bronze,_),Room(Bronze,_),Room(Bronze,_),
                Room(Copper,_),Room(Copper,_),Room(Copper,_),Room(Copper,_),
                Room(Desert,_),Room(Desert,_),Room(Desert,_),Room(Desert,_),
            ] => true,
            _ => false
        }
    }

    fn find_in(&self, pos: Position) -> Option<Amphipod> {
        self.positions.iter().position(|&p| p == pos).map(|idx| AMPHIPODS[idx])
    }

    fn kind_in(&self, pos: Position) -> Option<Kind> {
        self.find_in(pos).map(|a| a.kind)
    }

    fn is_path_clear(&self, path: &Vec<Position>) -> bool {
        for &pos in path {
            if self.find_in(pos).is_some() {
                return false;
            }
        }
        return true;
    }

    fn room_has_no_strangers(&self, kind: Kind) -> bool {
        let p1 = Position::Room(kind, Side::Top);
        let p2 = Position::Room(kind, Side::Bottom);
        let p3 = Position::Room(kind, Side::Bottomer);
        let p4 = Position::Room(kind, Side::Bottomest);
        self.kind_in(p1).unwrap_or(kind) == kind && 
        self.kind_in(p2).unwrap_or(kind) == kind && 
        self.kind_in(p3).unwrap_or(kind) == kind && 
        self.kind_in(p4).unwrap_or(kind) == kind
    }

    fn room_is_complete(&self, kind: Kind) -> bool {
        use Position::Room;
        let r = match kind {
            Kind::Amber => 0..4,
            Kind::Bronze => 4..8,
            Kind::Copper => 8..12,
            Kind::Desert => 12..16,
        };
        match self.positions[r] {
            [
                Room(k1,_),Room(k2,_),Room(k3,_),Room(k4,_),
            ] => k1 == k2 && k2 == k3 && k3 == k4 && k4 == kind,
            _ => false
        }
    }

    fn would_move_to(&self, who: Amphipod, to: Position) -> bool {
        let idx = who.index;
        let from = self.positions[idx];
        if from == to {
            // u silly
            return false;
        } else if let Position::Room(room_kind,_) = to {
            // to their destination room
            // as long as it contains no other amphipods
            return room_kind == who.kind && self.room_has_no_strangers(room_kind);
        } else if let (Position::Room(room_kind,_),Position::Hall(_)) = (from,to) {
            // from room to hall
            // unless room is complete
           return !self.room_is_complete(room_kind);
        }
        return false;
    }

    fn can_move_to(&self, who: Amphipod, to: Position, paths: &PathCache) -> (u32, bool) {
        let idx = who.index;
        let from = self.positions[idx];
        if from == to || !self.would_move_to(who, to) {
            return (u32::MAX, false);
        }
        let (cost, path) = from.path(to, paths);
        return (cost, self.is_path_clear(&path));
    }

    fn do_move(&self, who: Amphipod, to: Position, paths: &PathCache) -> State {
        let mut state = *self;
        let idx = who.index;
        let from = self.positions[idx];
        let (cost, _) = from.path(to, paths);
        state.positions[idx] = to;
        state.energy += who.kind.cost_multiplier() * cost;
        state
    }

    fn available_moves(&self, paths: &PathCache) -> Vec<(Amphipod, Position, u32)> {
        let mut moves = vec![];
        for a in AMPHIPODS {
            for p in POSITIONS {
                if let (cost, true) = self.can_move_to(a, p, paths) {
                    moves.push((a,p,cost));
                }
            }
        }
        moves.sort_by_key(|(_,_,cost)| *cost);
        moves
    }
}

fn read_input() -> State {
    let stdin = io::stdin();
    let mut kinds: Vec<Kind> = vec![];
    for line in stdin.lock().lines() {
        line.unwrap().chars().filter(char::is_ascii_alphabetic).for_each(|c| kinds.push(Kind::from_char(c)));
    }
    assert_eq!(16, kinds.len(), "Expected input with 16 letters");
    let mut positions = [Position::default(); 16];
    let mut next_a = vec![0,1,2,3];
    let mut next_b = vec![4,5,6,7];
    let mut next_c = vec![8,9,10,11];
    let mut next_d = vec![12,13,14,15];
    let mut room_positions = [
        Position::Room(Kind::Amber,  Side::Top),
        Position::Room(Kind::Bronze, Side::Top),
        Position::Room(Kind::Copper, Side::Top),
        Position::Room(Kind::Desert, Side::Top),
        Position::Room(Kind::Amber,  Side::Bottom),
        Position::Room(Kind::Bronze, Side::Bottom),
        Position::Room(Kind::Copper, Side::Bottom),
        Position::Room(Kind::Desert, Side::Bottom),
        Position::Room(Kind::Amber,  Side::Bottomer),
        Position::Room(Kind::Bronze, Side::Bottomer),
        Position::Room(Kind::Copper, Side::Bottomer),
        Position::Room(Kind::Desert, Side::Bottomer),
        Position::Room(Kind::Amber,  Side::Bottomest),
        Position::Room(Kind::Bronze, Side::Bottomest),
        Position::Room(Kind::Copper, Side::Bottomest),
        Position::Room(Kind::Desert, Side::Bottomest),
    ].iter();
    
    for c in kinds {
        let pos = *room_positions.next().unwrap();
        let nexts = match c {
            Kind::Amber => &mut next_a,
            Kind::Bronze => &mut next_b,
            Kind::Copper => &mut next_c,
            Kind::Desert => &mut next_d,
        };
        positions[nexts.remove(0)] = pos;
    }
    return State{energy: 0, positions: positions }
}

fn find_lowest_cost(start: &State) -> Vec<State> {
    let mut q = BinaryHeap::new();
    let paths = precompute_paths();
    let mut prev = HashMap::new();
    let mut lowest = State{energy:u32::MAX, positions: Default::default()};
    q.push(*start);
    while !q.is_empty() {
        let start = q.pop().unwrap();
        for (a,p,_) in start.available_moves(&paths) {
            let state = start.do_move(a,p,&paths);
            if state.energy > lowest.energy {
                continue;
            }
            if state.is_final() {
                lowest = state;
                println!("Found a final with {}", state.energy);
                prev.insert(state.positions, (state.energy, start.positions));
            } else if prev.get(&state.positions).map(|(e,_)| *e).unwrap_or(u32::MAX) > state.energy {
                prev.insert(state.positions, (state.energy, start.positions));
                q.push(state);
            }
        }
    }
    
    let mut states = vec![lowest];
    while prev.contains_key(&states[0].positions) {
        let (energy, positions) = *prev.get(&states[0].positions).unwrap();
        let state = State{energy: energy, positions: positions};
        states[0].energy = energy;
        states.insert(0, state);
    }
    states[0].energy = 0;
    states
}

fn main() {
    let state = read_input();
    println!("Start:\n{}", state);
    
    for state in find_lowest_cost(&state) {
        println!("{}Energy: {}", state, state.energy);
    }
}
