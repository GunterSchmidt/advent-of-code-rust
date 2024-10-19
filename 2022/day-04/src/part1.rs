/*!
# AoC 2022 Day 4 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/4>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This is a simple number comparison.

*/

pub fn solve_puzzle(input: &str) -> String {
    let mut result = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(',').unwrap();
        let (first_from, first_to) = first.split_once('-').unwrap();
        let (second_from, second_to) = second.split_once('-').unwrap();
        let first_from: u32 = first_from.parse().unwrap();
        let first_to: u32 = first_to.parse().unwrap();
        let second_from: u32 = second_from.parse().unwrap();
        let second_to: u32 = second_to.parse().unwrap();
        // is included?
        if (first_from <= second_from && first_to >= second_to)
            || (first_from >= second_from && first_to <= second_to)
        {
            result += 1;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!("2", solve_puzzle(input));
    }
}
