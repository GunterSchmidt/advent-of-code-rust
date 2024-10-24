/*!
# AoC 2022 Day 2 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/2>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This is a fairly fast solution using Enums.

*/

#[derive(Debug)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for RockPaperScissors {
    fn from(char: u8) -> Self {
        match char {
            b'A' => Self::Rock,
            b'B' => Self::Paper,
            b'C' => Self::Scissors,
            b'X' => Self::Rock,
            b'Y' => Self::Paper,
            b'Z' => Self::Scissors,
            _ => panic!("unknown character"),
        }
    }
}

pub fn solve_puzzle(input: &str) -> String {
    let strategy = parse_data(input);
    let winnings = calc_winnings(&strategy);
    winnings.to_string()
}

/// Winnings can be calculated easily, here the rules are hard coded.
fn calc_winnings(strategy: &[(RockPaperScissors, RockPaperScissors)]) -> usize {
    let mut winnings = 0;
    for (other, myself) in strategy.iter() {
        winnings += match other {
            RockPaperScissors::Rock => match myself {
                RockPaperScissors::Rock => 1 + 3,
                RockPaperScissors::Paper => 2 + 6,
                RockPaperScissors::Scissors => 3,
            },
            RockPaperScissors::Paper => match myself {
                RockPaperScissors::Rock => 1,
                RockPaperScissors::Paper => 2 + 3,
                RockPaperScissors::Scissors => 3 + 6,
            },
            RockPaperScissors::Scissors => match myself {
                RockPaperScissors::Rock => 1 + 6,
                RockPaperScissors::Paper => 2,
                RockPaperScissors::Scissors => 3 + 3,
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
        assert_eq!("15", solve_puzzle(input));
    }
}
