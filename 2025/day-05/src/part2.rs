/*!

# AoC 2025 Day 05 part 2
See the description of the puzzle at <https://adventofcode.com/2025/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

*/

use crate::parse_data;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut fresh = parse_data(input, false);
    fresh.merged_ranges_count().to_string()
}

/// The main function for this puzzle.
pub fn solve_puzzle_with_vec(input: &str) -> String {
    let mut fresh = parse_data(input, false);
    let mut total = 0;
    if let Some(merged) = fresh.merge_ranges() {
        for r in merged {
            total += r.end() - r.start() + 1;
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!("14", solve_puzzle(input));
    }
}
