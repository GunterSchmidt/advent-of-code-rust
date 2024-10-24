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

This is keys outer, directions inner.

*/

use crate::part1_hashmap::parse_input;

/// The logic is to find the first end of the sequence and count the steps required.
/// It is then assumed this step count will repeat.
/// After all numbers have been found the least common dominator needs to be calculated.
/// e.g. 2,3,9 = product 54, but 18 would be first
/// 2 and 3 have 6 as the least common dominator
/// 6 and 9 have 18 as the least common dominator
/// Why is this loop so slow?
/// Here a key is taken and all directions are done for that key.
/// Lookup uf direction seems to take longer because it is the greater table.
pub fn solve_puzzle_v1(input: &str) -> String {
    // The HashMap is used to store the left-right and top-bottom rules.
    // The three chars are converted to a number (key and direction values), so working will be fast.

    let (directions, map, start_keys) = parse_input(input);

    // find result steps for each start key
    let mut found_z_steps: Vec<u64> = Vec::with_capacity(start_keys.len());
    // this works because the individual number of steps till end is required
    for start_key in start_keys.iter() {
        let mut current = map.get(start_key).unwrap().clone();
        let mut result_steps = 1;
        for d in directions.iter().cycle() {
            let next_key = current[*d];
            if last_is_z(next_key) {
                found_z_steps.push(result_steps);
                break;
            }
            current = map.get(&next_key).unwrap().clone();
            result_steps += 1;
        }
    }

    // get the least common dominator
    let combined_result = least_common_denominator_with_multiplication(&found_z_steps);

    combined_result.to_string()
}

// source for this function: https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2023/rust/day-08/src/part2.rs
// also available in num_integer
pub fn least_common_denominator_with_multiplication(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = least_common_denominator_with_multiplication(&nums[1..]);
    a * b / greatest_common_divisor_of_two_numbers(a, b)
}

fn greatest_common_divisor_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    greatest_common_divisor_of_two_numbers(b, a % b)
}

#[inline]
pub fn last_is_z(key: u32) -> bool {
    key as u8 == b'Z'
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
        assert_eq!("6", solve_puzzle_v1(input));
    }
}
