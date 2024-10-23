/*!
# AoC 2023 Day 7 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/7>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This version does use an Enum for the Cards which is nicer style, but uses 10% more time.
The rank is identified in an array.
Later the hands must be sorted by rank, which takes 50% of all time.

*/

pub type CardArrEnum = [Card; 5];

// use an Enum for the cards with a value
// internally this will be compiled to u8 as Rust uses the smallest possible size
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Card {
    Undefined = 0,
    Joker = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

// Convert a char to a card
impl From<u8> for Card {
    fn from(c: u8) -> Self {
        match c {
            // 1 is the joker
            b'1' => Card::Joker,
            b'2' => Card::C2,
            b'3' => Card::C3,
            b'4' => Card::C4,
            b'5' => Card::C5,
            b'6' => Card::C6,
            b'7' => Card::C7,
            b'8' => Card::C8,
            b'9' => Card::C9,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => panic!("invalid card: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
// HandType with its default hand_type
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: CardArrEnum,
    pub bid: u32,
    pub hand_type: HandType,
}

impl Hand {
    fn new(cards: &[u8], bid: &[u8]) -> Hand {
        let cards = Self::hand_from_u8(cards);
        let hand_type = Hand::hand_type_counter_array(&cards);
        Hand {
            cards,
            bid: Self::atoi(bid) as u32,
            hand_type: hand_type,
        }
    }

    // Convert a string of length 5 to an array of cards
    fn hand_from_u8(s: &[u8]) -> CardArrEnum {
        if s.len() == 5 {
            let mut hand: CardArrEnum = [Card::Undefined; 5];
            for (i, c) in s.iter().enumerate() {
                hand[i] = Card::from(*c);
            }
            hand
        } else {
            panic!("hand '{:?}' must be of length 5", s);
        }
    }

    pub fn atoi(num: &[u8]) -> usize {
        let mut int = 0;
        for i in 0..num.len() {
            if num[i].is_ascii_digit() {
                int = int * 10 + (num[i] - b'0') as usize;
            }
        }
        int
    }

    /// looks good, but very slow
    #[allow(dead_code)]
    fn hand_type_counter_vector(cards: &CardArrEnum) -> HandType {
        // make vector of cards with its count and sort the vector descending by count
        let mut card_counter: Vec<(Card, i32)> = Vec::new();
        'card: for &card in cards.iter() {
            for (card_c, count) in card_counter.iter_mut() {
                if *card_c == card {
                    *count += 1;
                    continue 'card;
                }
            }
            card_counter.push((card, 1));
        }
        card_counter.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Evaluate
        match card_counter[0].1 {
            1 => return HandType::HighCard,
            2 => {
                if card_counter[1].1 == 2 {
                    return HandType::TwoPair;
                } else {
                    return HandType::OnePair;
                }
            }
            3 => {
                if card_counter[1].1 == 2 {
                    return HandType::FullHouse;
                } else {
                    return HandType::ThreeOfAKind;
                }
            }
            4 => return HandType::FourOfAKind,
            5 => return HandType::FiveOfAKind,
            _ => panic!("logic error"),
        }
    }

    /// faster array version
    fn hand_type_counter_array(cards: &CardArrEnum) -> HandType {
        // make vector of cards with its count and sort the vector descending by count
        let mut card_count = [0; 15];
        for &card in cards.iter() {
            card_count[card as usize] += 1;
        }

        // Evaluate
        // let mut is_pair = false;
        // let mut is_three_of_a_kind = false;
        let mut hand_type = HandType::HighCard;
        for &rank in card_count.iter().filter(|it| **it > 1) {
            match rank {
                2 => {
                    if hand_type == HandType::OnePair {
                        return HandType::TwoPair;
                    }
                    if hand_type == HandType::ThreeOfAKind {
                        return HandType::FullHouse;
                    }
                    hand_type = HandType::OnePair;
                }
                3 => {
                    if hand_type == HandType::OnePair {
                        return HandType::FullHouse;
                    }
                    hand_type = HandType::ThreeOfAKind;
                }
                4 => return HandType::FourOfAKind,
                5 => return HandType::FiveOfAKind,
                _ => panic!("logic error"),
            }
        }
        return hand_type;
    }
}

pub fn solve_puzzle(input: &str) -> String {
    // split input lines by whitespaces
    // extract a card and a bid
    // this will generate a list of hands with their default hand_type
    let data = input.as_bytes().split(|c| *c == b'\n').collect::<Vec<_>>();
    let mut hands: Vec<Hand> = Vec::with_capacity(data.len());
    for fields in data.iter() {
        if !fields.is_empty() {
            let hand = Hand::new(&fields[0..5], &fields[6..]);
            hands.push(hand);
        }
    }

    // sort hands by hand_type
    // if same hand_type, sort by cards
    // this sort uses 50% of all time
    hands.sort_unstable_by(|a, b| {
        if a.hand_type == b.hand_type {
            for i in 0..4 {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
            a.cards[4].cmp(&b.cards[4])
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });
    // dbg!(&hands);

    // calculate winnings, cards are sorted by rank
    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u32)
        .sum();

    winnings.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", solve_puzzle(input));
    }
}
