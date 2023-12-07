use crate::{ranking::HandType, IN_PART_2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Hand { cards }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Ace = 12,
    King = 11,
    Queen = 10,
    Jack = 9,
    Ten = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        HandType::from(*self).cmp(&HandType::from(*other))
    }
}

impl Card {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }

    pub fn all() -> [Self; 13] {
        [
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Jack,
            Card::Queen,
            Card::King,
            Card::Ace,
        ]
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if IN_PART_2.load(std::sync::atomic::Ordering::Relaxed) {
            if *self == Card::Jack {
                if *other == Card::Jack {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            } else if *other == Card::Jack {
                std::cmp::Ordering::Greater
            } else {
                (*self as u8).cmp(&(*other as u8))
            }
        } else {
            (*self as u8).cmp(&(*other as u8))
        }
    }
}
