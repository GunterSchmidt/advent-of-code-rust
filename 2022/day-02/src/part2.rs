/*!
# AoC 2022 Day 2 part 2
See the description of the puzzle at <https://adventofcode.com/2022/day/2>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This is a fairly fast solution using Enums.

*/

const ROCK_VALUE: usize = 1;
const PAPER_VALUE: usize = 2;
const SCISSORS_VALUE: usize = 3;

const WIN_VALUE: usize = 6;
const DRAW_VALUE: usize = 3;
const LOSE_VALUE: usize = 0;

#[derive(Debug)]
pub enum RockPaperScissors {
    RockLoose,
    PaperDraw,
    ScissorsWin,
}

impl From<u8> for RockPaperScissors {
    fn from(char: u8) -> Self {
        match char {
            b'A' => Self::RockLoose,
            b'B' => Self::PaperDraw,
            b'C' => Self::ScissorsWin,
            b'X' => Self::RockLoose,
            b'Y' => Self::PaperDraw,
            b'Z' => Self::ScissorsWin,
            _ => panic!("unknown character"),
        }
    }
}

pub fn solve_puzzle(input: &str) -> String {
    let strategy = parse_data(input);
    let winnings = calc_winnings(&strategy);
    winnings.to_string()
}

// There is no need to separate Lose, Draw, Win because the chosen object will determine the winnings.
// To make it more understandable, const values have been introduced.
fn calc_winnings(strategy: &[(RockPaperScissors, RockPaperScissors)]) -> usize {
    let mut winnings = 0;
    for (other, myself) in strategy.iter() {
        winnings += match other {
            RockPaperScissors::RockLoose => match myself {
                RockPaperScissors::RockLoose => SCISSORS_VALUE + LOSE_VALUE,
                RockPaperScissors::PaperDraw => ROCK_VALUE + DRAW_VALUE,
                RockPaperScissors::ScissorsWin => PAPER_VALUE + WIN_VALUE,
            },
            RockPaperScissors::PaperDraw => match myself {
                RockPaperScissors::RockLoose => ROCK_VALUE + LOSE_VALUE,
                RockPaperScissors::PaperDraw => PAPER_VALUE + DRAW_VALUE,
                RockPaperScissors::ScissorsWin => SCISSORS_VALUE + WIN_VALUE,
            },
            RockPaperScissors::ScissorsWin => match myself {
                RockPaperScissors::RockLoose => PAPER_VALUE + LOSE_VALUE,
                RockPaperScissors::PaperDraw => SCISSORS_VALUE + DRAW_VALUE,
                RockPaperScissors::ScissorsWin => ROCK_VALUE + WIN_VALUE,
            },
        };
    }
    winnings
}

fn parse_data(input: &str) -> Vec<(RockPaperScissors, RockPaperScissors)> {
    input
        .as_bytes()
        .split(|c| *c == b'\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some((
                RockPaperScissors::from(line[0]),
                RockPaperScissors::from(line[2]),
            ))
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "A Y
B X
C Z
";
        assert_eq!("12", solve_puzzle(input));
    }
}
