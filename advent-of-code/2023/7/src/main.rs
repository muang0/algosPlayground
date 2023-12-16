use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

struct Hand {
    cards: String,
    bid: u32,
    hand_type: HandType
}

impl Hand {
    fn new(hand_str: &str, bid: u32) -> Hand {
        let mut hand_map: HashMap<char, u8> = HashMap::new();
        for c in hand_str.chars() {
            if hand_map.contains_key(&c) {
                hand_map.insert(c, hand_map.get(&c).unwrap() + 1);
            } else {
                hand_map.insert(c, 1);
            }
        }
        // five of a kind
        if hand_map.keys().count() == 1 {
            return Hand{cards: hand_str.to_string(), hand_type: HandType::FiveOfAKind, bid};
        }
        // four of a kind
        if hand_map.iter().any(|(_, count)| *count == 4) {
            return Hand{cards: hand_str.to_string(), hand_type: HandType::FourOfAKind, bid};
        }
        // full house (three of a kind + two of a kind)
        if hand_map.keys().count() == 2 {
            return Hand{cards: hand_str.to_string(), hand_type: HandType::FullHouse, bid};
        }
        // three of a kind
        if hand_map.iter().any(|(_, count)| *count == 3) {
            return Hand{cards: hand_str.to_string(), hand_type: HandType::ThreeOfAKind, bid};
        }
        // two pair
        if hand_map.iter().filter(|(_, count)| **count == 2).count() == 2 {
            return Hand{cards: hand_str.to_string(), hand_type: HandType::TwoPair, bid};
        }
        // one pair
        if hand_map.iter().filter(|(_, count)| **count == 2).count() == 1 {
            return Hand{cards: hand_str.to_string(), hand_type: HandType::OnePair, bid};
        }
        // high card
        return Hand{cards: hand_str.to_string(), hand_type: HandType::HighCard, bid};
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            if self.hand_type == HandType::FiveOfAKind {
                return Ordering::Greater
            }
            if other.hand_type == HandType::FiveOfAKind {
                return Ordering::Less
            }
            if self.hand_type == HandType::FourOfAKind {
                return Ordering::Greater
            }
            if other.hand_type == HandType::FourOfAKind {
                return Ordering::Less
            }
            if self.hand_type == HandType::FullHouse {
                return Ordering::Greater
            }
            if other.hand_type == HandType::FullHouse {
                return Ordering::Less
            }
            if self.hand_type == HandType::ThreeOfAKind {
                return Ordering::Greater
            }
            if other.hand_type == HandType::ThreeOfAKind {
                return Ordering::Less
            }
            if self.hand_type == HandType::TwoPair {
                return Ordering::Greater
            }
            if other.hand_type == HandType::TwoPair {
                return Ordering::Less
            }
            if self.hand_type == HandType::OnePair {
                return Ordering::Greater
            }
            if other.hand_type == HandType::OnePair {
                return Ordering::Less
            }
        }
        for (index, c) in self.cards.chars().enumerate() {
            let other_c = other.cards.chars().nth(index).unwrap();
            if c != other_c {
                if c == 'A' { return Ordering::Greater }
                if other_c == 'A' { return Ordering::Less }

                if c == 'K' { return Ordering::Greater }
                if other_c == 'K' { return Ordering::Less }

                if c == 'Q' { return Ordering::Greater }
                if other_c == 'Q' { return Ordering::Less }

                if c == 'J' { return Ordering::Greater }
                if other_c == 'J' { return Ordering::Less }

                if c == 'T' { return Ordering::Greater }
                if other_c == 'T' { return Ordering::Less }

                if (c as u8) > (other_c as u8) {
                    return Ordering::Greater
                } else {
                    return Ordering::Less
                }
            }
        }
        return Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            if self.hand_type == HandType::FiveOfAKind {
                return Some(Ordering::Greater)
            }
            if other.hand_type == HandType::FiveOfAKind {
                return Some(Ordering::Less)
            }
            if self.hand_type == HandType::FourOfAKind {
                return Some(Ordering::Greater)
            }
            if other.hand_type == HandType::FourOfAKind {
                return Some(Ordering::Less)
            }
            if self.hand_type == HandType::FullHouse {
                return Some(Ordering::Greater)
            }
            if other.hand_type == HandType::FullHouse {
                return Some(Ordering::Less)
            }
            if self.hand_type == HandType::ThreeOfAKind {
                return Some(Ordering::Greater)
            }
            if other.hand_type == HandType::ThreeOfAKind {
                return Some(Ordering::Less)
            }
            if self.hand_type == HandType::TwoPair {
                return Some(Ordering::Greater)
            }
            if other.hand_type == HandType::TwoPair {
                return Some(Ordering::Less)
            }
            if self.hand_type == HandType::OnePair {
                return Some(Ordering::Greater)
            }
            if other.hand_type == HandType::OnePair {
                return Some(Ordering::Less)
            }
        }
        for (index, c) in self.cards.chars().enumerate() {
            let other_c = other.cards.chars().nth(index).unwrap();
            if c != other_c {
                if c == 'A' { return Some(Ordering::Greater) }
                if other_c == 'A' { return Some(Ordering::Less) }

                if c == 'K' { return Some(Ordering::Greater) }
                if other_c == 'K' { return Some(Ordering::Less) }

                if c == 'Q' { return Some(Ordering::Greater) }
                if other_c == 'Q' { return Some(Ordering::Less) }

                if c == 'J' { return Some(Ordering::Greater) }
                if other_c == 'J' { return Some(Ordering::Less) }

                if c == 'T' { return Some(Ordering::Greater) }
                if other_c == 'T' { return Some(Ordering::Less) }

                if (c as u8) > (other_c as u8) {
                    return Some(Ordering::Greater)
                } else {
                    return Some(Ordering::Less)
                }
            }
        }
        return Some(Ordering::Equal)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.split("\n") {
        let hand_str = line.split(" ").nth(0).unwrap();
        let bid: u32 = line.split(" ").nth(1).unwrap().parse().unwrap();
        hands.push(Hand::new(hand_str, bid));
    }
    hands.sort();
    let mut total_winnings: u64 = 0;
    for (index, hand) in hands.iter().enumerate() {
        total_winnings += (index as u64 + 1) * (hand.bid as u64);
    }
    println!("total winnings: {:#}", total_winnings);
}
