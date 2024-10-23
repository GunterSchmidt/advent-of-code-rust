/*!
# AoC 2023 Day 4 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/4>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This version uses classical string parsing as performance reference.

*/

struct Card {
    winning_numbers: Vec<u8>,
    drawn_numbers: Vec<u8>,
}

impl Card {
    // check each drawn number if it is in the list of winning numbers
    // This is implemented as vec, as looping over small vecs is faster than hashing.
    fn points(&self) -> u32 {
        let count = self
            .drawn_numbers
            .iter()
            .filter(|&x| self.winning_numbers.contains(x))
            .count() as u32;

        if count > 2 {
            // double
            2u32.pow(count - 1)
        } else {
            count
        }
    }
}

pub fn solve_puzzle(input: &str) -> String {
    let cards = parse_card(input);
    let total = cards.iter().map(|card| card.points()).sum::<u32>();
    total.to_string()
}

fn parse_card(input: &str) -> Vec<Card> {
    let cards: Vec<Card> = input
        .lines()
        .filter_map(|line| {
            let pos_win: usize;
            let pos_win_test = line.find(':'); //.expect("Expected 'Card xx:'") + 1;
            match pos_win_test {
                Some(p) => pos_win = p + 2,
                None => return None,
            }

            let pos_draw = line[pos_win..].find('|').expect("Expected divider '|'") + pos_win;
            // dbg!(pos_win, pos_draw);

            // let x: Vec<String> = line[pos_win..pos_draw]
            //     .split_whitespace()
            //     .map(|it| it.to_owned())
            //     .collect();
            // dbg!(x);

            let winning_numbers = line[pos_win..pos_draw]
                .split_whitespace()
                .map(|num| num.parse::<u8>().expect("Expected number"))
                .collect();
            let drawn_numbers = line[pos_draw + 1..]
                .split_whitespace()
                .map(|num| num.parse::<u8>().expect("Expected number"))
                .collect();

            let card = Card {
                winning_numbers: winning_numbers,
                drawn_numbers: drawn_numbers,
            };

            Some(card)
        })
        .collect();

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", solve_puzzle(input));
    }
}
