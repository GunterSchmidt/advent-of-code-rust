/*!

# AoC 2025 Day 03 part 2
See the description of the puzzle at <https://adventofcode.com/2025/day/3>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

*/

use crate::parse_data;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let data = parse_data(input);
    // dbg!(&data);

    // calc joltage
    let mut sum = 0;
    for b in data {
        sum += b.joltage_twelve();
        // let j = b.joltage_twelve();
        // println!("{b}: {j}");
        // sum += j;
    }

    //     dial.count_zero_pos.to_string()
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!("3121910778619", solve_puzzle(input));
    }
}
