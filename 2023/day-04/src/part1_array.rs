/*!
# AoC 2023 Day 4 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/4>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This version uses arrays for the cards. While somewhat unpractical in real life,
this gives another 30% speed boost.
A test where I removed the struct Card and did the calculation while parsing yielded
actually a bit slower performance.

*/

const N_WINNING_CARDS: usize = 10;
const N_DRAWN_CARDS: usize = 25;

struct Card {
    winning_numbers: [u8; N_WINNING_CARDS],
    drawn_numbers: [u8; N_DRAWN_CARDS],
    n_winning_numbers: usize,
    n_drawn_numbers: usize,
}

impl Card {
    // check each drawn number if it is in the list of winning numbers
    // This is implemented as vec, as looping over small vecs is faster than hashing.
    fn points(&self) -> u32 {
        let count = self.drawn_numbers[0..self.n_drawn_numbers]
            .iter()
            .filter(|x| self.winning_numbers[0..self.n_winning_numbers].contains(x))
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
    let cards = parse_cards(input);
    let total = cards.iter().map(|card| card.points()).sum::<u32>();
    total.to_string()
}

/// parses the input and returns all cards
fn parse_cards(input: &str) -> Vec<Card> {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let b_line = line.as_bytes();
            let pos_win = b_line
                .iter()
                .position(|&it| it == b':')
                .expect("Expected ':'")
                + 2;
            let pos_draw = b_line[pos_win..]
                .iter()
                .position(|&it| it == b'|')
                .expect("Expected divider '|'")
                + pos_win;
            // dbg!(pos_win, pos_draw);

            let mut i_win = 0;
            let mut winning_numbers = [0; N_WINNING_CARDS];
            for number in b_line[pos_win..pos_draw].split(|&it| it == b' ') {
                match number.len() {
                    0 => continue,
                    1 => {
                        winning_numbers[i_win] = number[0] - b'0';
                        i_win += 1
                    }
                    2 => {
                        winning_numbers[i_win] = (number[0] - b'0') * 10 + number[1] - b'0';
                        i_win += 1
                    }
                    _ => panic!("read error"),
                }
            }
            let mut i_draw = 0;
            let mut drawn_numbers = [0; N_DRAWN_CARDS];
            for number in b_line[pos_draw + 1..].split(|&it| it == b' ') {
                match number.len() {
                    0 => continue,
                    1 => {
                        drawn_numbers[i_draw] = number[0] - b'0';
                        i_draw += 1
                    }
                    2 => {
                        drawn_numbers[i_draw] = (number[0] - b'0') * 10 + number[1] - b'0';
                        i_draw += 1
                    }
                    _ => panic!("read error"),
                }
            }

            let card = Card {
                winning_numbers: winning_numbers,
                drawn_numbers: drawn_numbers,
                n_winning_numbers: i_win,
                n_drawn_numbers: i_draw,
            };

            card
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
