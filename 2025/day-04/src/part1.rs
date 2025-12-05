/*!

# AoC 2025 Day 04 part 1
See the description of the puzzle at <https://adventofcode.com/2025/day/4>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

*/

use crate::parse_data;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let floor = parse_data(input);

    // count adjacent fields of paper roll
    let mut accessible_rolls = 0;
    for row in 1..floor.size + 1 {
        for col in 1..floor.size + 1 {
            if floor.map[row][col] == 1 {
                // Just count all. An additional if to check if 4 is reached already probably takes longer.
                let cnt_adj_rolls = floor.map[row - 1][col - 1]
                    + floor.map[row - 1][col]
                    + floor.map[row - 1][col + 1]
                    + floor.map[row][col - 1]
                    + floor.map[row][col + 1]
                    + floor.map[row + 1][col - 1]
                    + floor.map[row + 1][col]
                    + floor.map[row + 1][col + 1];
                if cnt_adj_rolls < 4 {
                    // dbg!(row, col);
                    accessible_rolls += 1;
                }
            }
        }
    }

    accessible_rolls.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!("13", solve_puzzle(input));
    }
}
