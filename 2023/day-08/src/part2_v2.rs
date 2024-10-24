/*!
# AoC 2023 Day 8 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/8>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Simultaneous path requires a double loop:
For each key all directions need to be cycled.
Since the direction is the larger table (780 entries)
and keys is the smaller table (6 entries), it is less
efficient to use the keys in the outer loop.

This is directions outer, keys inner, which is twice as fast.

*/

use crate::{
    part1_hashmap::parse_input,
    part2_v1::{last_is_z, least_common_denominator_with_multiplication},
};

/// This loop is almost twice as fast af part2_v1, reason unknown
/// Please drop me a note if you figure out why this is.
/// Here the direction is taken and this direction is calculated for all simultaneous path.
pub fn solve_puzzle_v2(input: &str) -> String {
    let (directions, map, start_keys) = parse_input(input);

    let key_count = start_keys.len();
    let mut found_z_steps: Vec<u64> = vec![0; key_count];

    let mut curr_left_right: Vec<[u32; 2]> = start_keys
        .iter()
        .map(|k| map.get(k).unwrap().clone())
        .collect();

    // repeat direction until ZZZ is reached
    let mut result_steps: u64 = 1;
    let mut key_i: Vec<usize> = (0..key_count).collect();
    'outer: for d in directions.iter().cycle() {
        // check: next step all end keys?
        let mut i: i32 = 0;
        while (i as usize) < key_i.len() {
            let k = key_i[i as usize];
            let next_key = curr_left_right[k][*d];
            if last_is_z(next_key) {
                found_z_steps[k] = result_steps;
                // remove this key, we have the loop result
                let index = key_i.iter().position(|&r| r == k).unwrap();
                key_i.remove(index);
                if key_i.is_empty() {
                    break 'outer;
                }
                i -= 1;
            }
            curr_left_right[k] = map.get(&next_key).unwrap().clone();
            i += 1;
        }
        result_steps += 1;
    }
    let combined_result = least_common_denominator_with_multiplication(&found_z_steps);

    combined_result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", solve_puzzle_v2(input));
    }
}
