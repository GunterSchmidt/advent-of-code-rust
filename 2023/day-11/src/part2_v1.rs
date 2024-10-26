/*!
# AoC 2023 Day 11 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/11>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Like part1 but with parsing to 64-bit numbers. 50% slower, even with EXPANSION_FACTOR = 1.

*/

use crate::part1_v1::parse_universe;
const EXPANSION_FACTOR: u32 = 1; // 999_999;

pub fn solve_puzzle(input: &str) -> String {
    let galaxies = parse_universe(input, EXPANSION_FACTOR);
    // match universes and find shortest paths
    let mut sum_len_paths = 0;
    for (i_first, (first_row, first_col)) in galaxies.iter().enumerate() {
        let sum = galaxies
            .iter()
            .skip(i_first)
            .map(|(second_row, second_col)| {
                (*first_row as i64 - *second_row as i64).abs()
                    + (*first_col as i64 - *second_col as i64).abs()
            })
            .sum::<i64>() as u64;
        sum_len_paths += sum;
    }

    sum_len_paths.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // this test requires EXPANSION_FACTOR = 99
    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("8410", solve_puzzle(input));
    }
}
