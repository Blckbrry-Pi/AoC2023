use crate::{hand::{Card, Hand}, IN_PART_2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandTypeDiscriminant {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandType {
    hand: Hand,
    disc: HandTypeDiscriminant,
}

pub fn all_eq(cards: &[Card]) -> bool {
    cards.iter().skip(1).all(|&card| card == cards[0])
}

// Regular Hand -> HandType conversion
pub fn part_1_from(hand: Hand) -> HandType {
    let mut cards = hand.cards;
    cards.sort();

    // all equal
    if all_eq(&cards) {
        return HandType {
            hand,
            disc: HandTypeDiscriminant::FiveOfAKind,
        }
    }

    // first 4 or last 4 equal
    if all_eq(&cards[1..]) || all_eq(&cards[..4]) {
        return HandType {
            hand,
            disc: HandTypeDiscriminant::FourOfAKind,
        }
    }

    // first 3, mid 3, or last 3 equal
    if all_eq(&cards[..3]) || all_eq(&cards[1..4]) || all_eq(&cards[2..]) {
        // first 2 and last 2 equal
        if all_eq(&cards[..2]) && all_eq(&cards[3..]) {
            return HandType {
                hand,
                disc: HandTypeDiscriminant::FullHouse,
            }
        } else {
            return HandType {
                hand,
                disc: HandTypeDiscriminant::ThreeOfAKind,
            }
        }
    }


    let fst_2_eq = cards[0] == cards[1];
    let m_l_2_eq = cards[1] == cards[2];
    let m_r_2_eq = cards[2] == cards[3];
    let lst_2_eq = cards[3] == cards[4];

    let is_two_pair_with_fst_pair_l = fst_2_eq && (m_r_2_eq || lst_2_eq);
    let is_two_pair_with_fst_pair_r = m_l_2_eq && lst_2_eq;

    if is_two_pair_with_fst_pair_l || is_two_pair_with_fst_pair_r {
        return HandType {
            hand,
            disc: HandTypeDiscriminant::TwoPair,
        }
    } 

    if fst_2_eq || m_l_2_eq || m_r_2_eq || lst_2_eq {
        return HandType {
            hand,
            disc: HandTypeDiscriminant::OnePair,
        }
    }

    HandType {
        hand,
        disc: HandTypeDiscriminant::HighCard,
    }
}

// This is incredibly hacky and I love it.

impl From<Hand> for HandType {
    fn from(hand: Hand) -> Self {
        if IN_PART_2.load(std::sync::atomic::Ordering::Relaxed) {
            // If there's a jack, try all the other cards that it could be (and recurse!)
            for i in 0..5 {
                // If card is a jack, get the best of all the other hands where
                // the card is any other card
                if hand.cards[i] == Card::Jack {
                    // All non-jack cards
                    let all_non_jack = Card::all().into_iter().filter(|&card| card != Card::Jack);
                    
                    // All hands where the jack is replaced with the card type
                    // for discriminant comparison but not for hand comparison
                    let all_non_hand_types = all_non_jack.map(|card_type| {
                        let mut cards = hand.cards;
                        cards[i] = card_type;
                        let hand_type = HandType::from(Hand::new(cards));
                        HandType { hand, disc: hand_type.disc }
                    });

                    // Best of the above
                    return all_non_hand_types.max().unwrap();
                }
            }
        }

        part_1_from(hand)
    }
} 

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.disc.cmp(&other.disc) {
            std::cmp::Ordering::Equal => {
                for (card, other_card) in self.hand.cards.iter().zip(other.hand.cards.iter()) {
                    match card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        v => return v,
                    }
                }
                std::cmp::Ordering::Equal
            },
            v => v,
        }
    }
}
