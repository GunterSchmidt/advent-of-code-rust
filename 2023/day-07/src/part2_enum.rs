/*!
# AoC 2023 Day 7 part 2
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

use crate::part1_enum::{Card, CardArrEnum, Hand, HandType};

// Convert a string of length 5 to an array of cards
impl Card {
    pub fn hand_from_u8_with_joker(s: &[u8]) -> CardArrEnum {
        if s.len() == 5 {
            let mut hand: CardArrEnum = [Card::Undefined; 5];
            for (i, c) in s.iter().enumerate() {
                let c = if *c == b'J' { b'1' } else { *c };
                hand[i] = Card::from(c);
            }
            hand
        } else {
            panic!("hand '{:?}' must be of length 5", s);
        }
    }
}

impl Hand {
    fn new_with_joker_enum(cards: CardArrEnum, bid: &[u8]) -> Hand {
        let hand_type = Hand::hand_type_counter_array_with_joker(&cards);
        Hand {
            cards,
            bid: Self::atoi(bid) as u32,
            hand_type: hand_type,
        }
    }

    /// faster array version
    fn hand_type_counter_array_with_joker(cards: &CardArrEnum) -> HandType {
        // make vector of cards with its count and sort the vector descending by count
        let mut card_count = [0; 15];
        let mut joker_count = 0;
        for &card in cards.iter() {
            if card == Card::Joker {
                joker_count += 1;
            } else {
                card_count[card as usize] += 1;
            }
        }
        if joker_count > 3 {
            return HandType::FiveOfAKind;
        }

        // Evaluate
        let mut hand_type = HandType::HighCard;
        for &rank in card_count.iter().filter(|it| **it > 1) {
            match rank {
                2 => {
                    if hand_type == HandType::OnePair {
                        hand_type = HandType::TwoPair;
                        break;
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
                4 => {
                    hand_type = HandType::FourOfAKind;
                    break;
                }
                5 => return HandType::FiveOfAKind,
                _ => panic!("logic error"),
            }
        }
        match joker_count {
            0 => return hand_type,
            1 => match hand_type {
                HandType::HighCard => return HandType::OnePair,
                HandType::OnePair => return HandType::ThreeOfAKind,
                HandType::TwoPair => return HandType::FullHouse,
                HandType::ThreeOfAKind => return HandType::FourOfAKind,
                HandType::FourOfAKind => return HandType::FiveOfAKind,
                _ => panic!("logic error"),
            },
            2 => match hand_type {
                HandType::HighCard => return HandType::ThreeOfAKind,
                HandType::OnePair => return HandType::FourOfAKind,
                HandType::ThreeOfAKind => return HandType::FiveOfAKind,
                _ => panic!("logic error"),
            },
            3 => match hand_type {
                HandType::HighCard => return HandType::FourOfAKind,
                HandType::OnePair => return HandType::FiveOfAKind,
                _ => panic!("logic error"),
            },
            _ => panic!("logic error"),
        }
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
            let cards = Card::hand_from_u8_with_joker(&fields[0..5]);
            let hand = Hand::new_with_joker_enum(cards, &fields[6..]);
            hands.push(hand);
        }
    }
    // dbg!(&hands);

    // sort hands by hand_type
    // if same hand_type, sort by cards
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
        assert_eq!("5905", solve_puzzle(input));
    }
}
