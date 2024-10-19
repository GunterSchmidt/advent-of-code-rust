/*!
# AoC 2022 Day 3 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/3>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Find the same char in two strings.
Simple loop search for each letter in first string until matching char is found.

*/

/// This is the main function to solve the puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut result = 0;
    for line in input.lines() {
        result += search_common_item(line);
    }

    result.to_string()
}

/// Searches for the common item.
/// TODO This seems inefficient as the search could be cut on both ends if letters were sorted
#[inline]
fn search_common_item(line: &str) -> usize {
    let pos = line.len() / 2;
    let (first, second) = line.split_at(pos);
    let first = first.as_bytes();
    let second = second.as_bytes();
    for &letter in first.iter() {
        if second.iter().any(|&it| it == letter) {
            return if letter < 95 {
                (letter - 38) as usize
            } else {
                (letter - 96) as usize
            };
        }
    }
    panic!("no common item found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!("157", solve_puzzle(input));
    }
}
