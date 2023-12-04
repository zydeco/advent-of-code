use std::io::{self, BufRead};

struct Card {
    winning_numbers: u128,
    numbers_you_have: u128,
}

fn to_bits(s: &str) -> u128 {
    s.split_ascii_whitespace()
        .flat_map(str::parse::<u8>)
        .fold(0u128, |sum, x| sum | (1u128 << x))
}
impl Card {
    fn from_string(s: String) -> Card {
        assert!(s.starts_with("Card "));
        let s = s.split(':').skip(1).next().unwrap();
        let mut parts = s.split('|');
        Card {
            winning_numbers: to_bits(parts.next().unwrap()),
            numbers_you_have: to_bits(parts.next().unwrap()),
        }
    }

    fn matches(&self) -> usize {
        (self.winning_numbers & self.numbers_you_have).count_ones() as usize
    }

    fn points(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        }
        return 1 << (matches - 1);
    }
}
fn main() {
    let cards: Vec<Card> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(Card::from_string)
        .collect();

    let points = cards.iter().map(Card::points).fold(0, |sum, p| sum + p);
    println!("Points: {}", points);

    // part two
    let mut copies = vec![0; cards.len()];
    for (idx, card) in cards.iter().enumerate() {
        copies[idx] += 1;
        for n in 0..card.matches() {
            copies[idx + 1 + n] += copies[idx];
        }
    }
    let total_cards = copies.iter().fold(0, |sum, x| sum + x);
    println!("Total: {}", total_cards);
}
