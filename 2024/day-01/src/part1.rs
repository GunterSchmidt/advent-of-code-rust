/*!

# AoC 2024 Day 01 part 1
See the description of the puzzle at <https://adventofcode.com/2024/day/01>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Compare two lists ascending and calculate delta.

This code is neither rusty nor concise. It is fast.

The data is parsed as_bytes() allowing to iter ASCII chars fast.
Instead of split a match function is used to save some time.

*/

use crate::parse_data;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let (data_first, data_second) = parse_data(input);

    let mut delta = 0;
    for i in 0..data_first.len() {
        let d = (data_second[i] - data_first[i]).abs();
        delta += d;
    }

    delta.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!("11", solve_puzzle(input));
    }
}
