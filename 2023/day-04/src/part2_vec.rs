/*!
# AoC 2023 Day 4 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/4>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This uses the same parser as part1_vec, so performance is similar.

*/

use crate::part1_vec::parse_cards;

// won cards are counted in an array and then summed up
pub fn solve_puzzle(input: &str) -> String {
    let cards = parse_cards(input);

    let len = cards.len();

    let total: u32;
    if len < 250 {
        // use array, but this gives no time advantage
        let mut won_cards = [1u32; 250];
        // loop all cards and collect won cards
        for (index, card) in cards.iter().enumerate() {
            let count = card.winning_numbers_count() as usize;
            for i in 0..count {
                won_cards[index + i + 1] += won_cards[index];
            }
        }
        // sum over won cards
        total = won_cards.iter().sum();
    } else {
        let mut won_cards: Vec<u32> = vec![1; len];
        // loop all cards and collect won cards
        for (index, card) in cards.iter().enumerate() {
            let count = card.winning_numbers_count() as usize;
            for i in 0..count {
                won_cards[index + i + 1] += won_cards[index];
            }
        }
        // sum over won cards
        total = won_cards.iter().sum();
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", solve_puzzle(input));
    }
}
