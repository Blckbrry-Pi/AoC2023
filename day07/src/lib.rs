
use hand::Card;

pub mod hand;
pub mod ranking;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandBidPair(pub ranking::HandType, pub usize);

impl PartialOrd for HandBidPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandBidPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl HandBidPair {
    pub fn parse_line(line: &str) -> Self {
        let cards: Vec<_> = line.chars().take(5)
            .map(|c| Card::from_char(c).unwrap())
            .collect();

        let hand = hand::Hand::new([cards[0], cards[1], cards[2], cards[3], cards[4]]);
        let bid = line.split(' ').last().unwrap().parse().unwrap();

        HandBidPair(ranking::HandType::from(hand), bid)
    }
}


use std::sync::atomic::AtomicBool;
pub static IN_PART_2: AtomicBool = AtomicBool::new(false);

