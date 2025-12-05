/*!

# AoC 2025 Day 04 part 2
See the description of the puzzle at <https://adventofcode.com/2025/day/4>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

*/

use crate::parse_data;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut floor = parse_data(input);

    // Count adjacent fields of paper roll.
    let mut total = 0;
    // Loop over whole array until no more paper rolls to process are found.
    // One could write more complex code to only check fields again, which were affected by a removed roll.
    // But each removed roll would affect 8 fields and as such these 8 fields would be checked multiple times.
    // This could be addressed by extending the map and storing the surrounding fields.
    // All in all probably not a performance benefit compared to extremely fast array operation.
    loop {
        let mut accessible_rolls = 0;
        for row in 1..floor.size + 1 {
            for col in 1..floor.size + 1 {
                if floor.map[row][col] == 1 {
                    // Just count all. An additional if probably takes longer.
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
                        // removing paper roll immediately will free following paper rolls early
                        floor.map[row][col] = 0;
                    }
                }
            }
            // println!("range = {:?}", halves_range);
        }
        if accessible_rolls == 0 {
            break;
        }
        total += accessible_rolls;
    }

    //     dial.count_zero_pos.to_string()
    total.to_string()
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
        assert_eq!("43", solve_puzzle(input));
    }
}
