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

impl From<char> for RockPaperScissors {
    fn from(char: char) -> Self {
        match char {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
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
    let mut strategy: Vec<(RockPaperScissors, RockPaperScissors)> = Vec::new();
    for line in input.lines() {
        if let Some((other, myself)) = line.split_once(' ') {
            strategy.push((
                RockPaperScissors::from(other.chars().next().unwrap()),
                RockPaperScissors::from(myself.chars().next().unwrap()),
            ));
        }
    }

    strategy
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
