/*!
# AoC 2022 Day 3 part 2
See the description of the puzzle at <https://adventofcode.com/2022/day/3>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This time an array is build to store if a letter is in the first two strings,
then the third string is checked. This is 2.5 times faster.

*/

/// This is the main function to solve the puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut result = 0;
    let mut line_set = Vec::new();
    for line in input.lines() {
        line_set.push(line);
        // grab three lines, then search
        if line_set.len() == 3 {
            result += search_common_item_array(&line_set);
            line_set.clear();
        }
    }

    result.to_string()
}

/// Using array to identify unique shared letter.
#[inline]
fn search_common_item_array(data: &[&str]) -> usize {
    let first = data[0].as_bytes();
    let second = data[1].as_bytes();
    let third = data[2].as_bytes();
    // store count in array using ASCII key
    let mut exists = [0; 125];
    for &letter in first.iter() {
        exists[letter as usize] = 1;
    }
    for &letter in second.iter() {
        if exists[letter as usize] == 1 {
            exists[letter as usize] = 2
        };
    }
    for &letter in third.iter() {
        if exists[letter as usize] == 2 {
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
        assert_eq!("70", solve_puzzle(input));
    }
}
