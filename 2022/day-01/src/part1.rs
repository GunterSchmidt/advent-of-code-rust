/*!
# AoC 2022 Day 1 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/1>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This is a simple sum function.

*/

pub type Calory = u32;
pub struct Elf {
    pub calories: Vec<Calory>,
}

impl Elf {
    pub fn sum_calories(&self) -> Calory {
        self.calories.iter().sum()
    }

    /// reads the input and converts it into a vector of elfes
    pub fn parse_input(input: &str) -> Vec<Elf> {
        let mut elfes = Vec::new();
        let mut calories = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                elfes.push(Elf { calories });
                calories = Vec::new();
            } else {
                calories.push(line.parse::<Calory>().unwrap());
            }
        }
        elfes.push(Elf { calories });

        elfes
    }
}

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let elfes = Elf::parse_input(input);

    let mut max = 0;
    for elf in elfes.iter() {
        max = max.max(elf.sum_calories());
    }

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!("24000", solve_puzzle(input));
    }
}
