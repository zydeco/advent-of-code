use std::{
    char,
    collections::HashMap,
    fmt::Debug,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Invalid card {}", c),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::J => write!(f, "J"),
            Self::_2 => write!(f, "2"),
            Self::_3 => write!(f, "3"),
            Self::_4 => write!(f, "4"),
            Self::_5 => write!(f, "5"),
            Self::_6 => write!(f, "6"),
            Self::_7 => write!(f, "7"),
            Self::_8 => write!(f, "8"),
            Self::_9 => write!(f, "9"),
            Self::T => write!(f, "T"),
            Self::Q => write!(f, "Q"),
            Self::K => write!(f, "K"),
            Self::A => write!(f, "A"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn from_str(str: &str) -> Hand {
        assert_eq!(5, str.len());
        let mut chars = str.chars();
        Hand {
            cards: [
                Card::from_char(chars.next().unwrap()),
                Card::from_char(chars.next().unwrap()),
                Card::from_char(chars.next().unwrap()),
                Card::from_char(chars.next().unwrap()),
                Card::from_char(chars.next().unwrap()),
            ],
        }
    }

    fn handType(&self) -> HandType {
        // count without jokers
        let by_type: HashMap<Card, usize> = self
            .cards
            .iter()
            .filter(|&&card| card != Card::J)
            .fold(HashMap::new(), |mut map, card| {
                *map.entry(*card).or_insert(0) += 1;
                map
            });
        let jokers = self.cards.iter().filter(|&&card| card == Card::J).count();
        let mut counts: Vec<usize> = by_type.into_values().collect();
        counts.sort();
        counts.reverse();
        let counts3 = [
            // add jokers to largest group
            *counts.get(0).unwrap_or(&0) + jokers,
            *counts.get(1).unwrap_or(&0),
            *counts.get(2).unwrap_or(&0),
        ];

        match counts3 {
            [5, 0, 0] => HandType::FiveOfAKind,
            [4, 1, 0] => HandType::FourOfAKind,
            [3, 2, 0] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1] => HandType::OnePair,
            [1, 1, 1] => HandType::HighCard,
            _ => panic!("Impossible hand {:?} (counts = {:?})", self, counts3),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let t1 = self.handType();
        let t2 = other.handType();
        if t1 != t2 {
            return t1.cmp(&t2);
        }

        // same type, compare card by card
        for (c1, c2) in (0..5).map(|i| (self.cards[i], other.cards[i])) {
            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input() -> Vec<(Hand, i32)> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            (
                Hand::from_str(&line[0..5]),
                i32::from_str_radix(&line[6..], 10).unwrap(),
            )
        })
        .collect()
}
fn main() {
    let mut hands = read_input();
    hands.sort_by_key(|&(hand, _)| hand);

    // part 1
    let winnings: i32 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, bid))| acc + (1 + idx as i32) * bid);
    println!("Total winnings: {}", winnings);
}
