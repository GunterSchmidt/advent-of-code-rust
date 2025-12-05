/*!

# AoC 2025 Day 05 part 1
See the description of the puzzle at <https://adventofcode.com/2025/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

*/

use crate::parse_data;

/// The main function for this puzzle. Sorting the ranges ascending for early search termination.
/// Possibly faster to do a binary search first, which is more complex here as the ranges are overlapping.
/// Not pursued since result is in microsecond range.
pub fn solve_puzzle(input: &str) -> String {
    let mut data = parse_data(input, true);
    data.sort_fresh();
    // dbg!(&data);

    // count fresh
    let mut cnt_fresh = 0;
    for id in data.available.iter() {
        for r in data.fresh.iter() {
            if id >= r.start() {
                if id <= r.end() {
                    cnt_fresh += 1;
                    break;
                }
            } else {
                // id is smaller than all following start of ranges
                break;
            }
        }
    }

    cnt_fresh.to_string()
}

/// The main function for this puzzle. Just looping over all ranges.
pub fn solve_puzzle_simple(input: &str) -> String {
    let data = parse_data(input, true);
    // dbg!(&data);

    // count fresh
    let mut cnt_fresh = 0;
    for id in data.available.iter() {
        for r in data.fresh.iter() {
            if r.contains(id) {
                cnt_fresh += 1;
                break;
            }
        }
    }

    cnt_fresh.to_string()
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
        assert_eq!("3", solve_puzzle(input));
    }
}
