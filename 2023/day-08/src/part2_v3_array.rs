/*!
# AoC 2023 Day 8 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/8>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

An array is about 30% faster but this is only an option, if the actual
size of the data known and limited in size. Large arrays can lead to a
stack overflow error. Try using u8 or u16 instead of usize. Casting is
inexpensive.

*/

use crate::part2_v1::least_common_denominator_with_multiplication;

pub fn solve_puzzle_array(input: &str) -> String {
    // The key only needs to be u8 but needs to be casted to usize for array access.
    // In my tests the speed was:
    // usize: 0.371 ms
    // u32: 0.320 ms
    // u16: 0.306 ms
    // u8: 0.299 ms
    type Key = u8;
    const END_KEY: Key = (b'Z' - b'A') as Key;
    let mut lines = input.lines();

    // parse data
    // first line is left-right rule
    let left_right = lines.next().unwrap();
    // direction is a vector for left-right, left = true
    let directions: Vec<usize> = left_right
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    lines.next();

    // create array for all 3 letter combinations
    // array of left right (result) [[u8; 3][u8; 3]] for all letters of the key
    let mut map_array = [[[[[0; 3]; 2]; 26]; 26]; 26];
    let mut start_keys = Vec::with_capacity(10);
    while let Some(line) = lines.next() {
        let line = line
            .as_bytes()
            .iter()
            .map(|c| {
                match c {
                    // get letters for key and subtract A so array is 0-25
                    // numbers are only used in the test example and are merged in letter range
                    b'1'..=b'9' => (*c - 44) as Key,
                    b'A'..=b'Z' => (*c - b'A') as Key,
                    _ => *c as Key,
                }
            })
            .collect::<Vec<_>>();

        let key: [Key; 3] = line[0..3].try_into().unwrap();
        let left: [Key; 3] = line[7..10].try_into().unwrap();
        let right: [Key; 3] = line[12..15].try_into().unwrap();

        if key[2] == 0 {
            start_keys.push(key);
        }
        map_array[key[0] as usize][key[1] as usize][key[2] as usize] = [left, right];
    }

    // find pathes ends
    let key_count = start_keys.len();
    let mut found_z_steps: Vec<u64> = vec![0; key_count];

    let mut curr_left_right = start_keys
        .iter()
        .map(|k| map_array[k[0] as usize][k[1] as usize][k[2] as usize])
        .collect::<Vec<_>>();

    // repeat direction until ZZZ is reached
    let mut result_steps: u64 = 1;
    let mut key_i: Vec<usize> = (0..key_count).collect();
    'outer: for d in directions.iter().cycle() {
        // check: next step all end keys?
        let mut i: i32 = 0;
        while (i as usize) < key_i.len() {
            let k = key_i[i as usize];
            let next_key = curr_left_right[k][*d];
            if next_key[2] == END_KEY {
                found_z_steps[k] = result_steps;
                // dbg!(&found_z_steps);
                // remove this key, we have the loop result
                let index = key_i.iter().position(|&r| r == k).unwrap();
                key_i.remove(index);
                if key_i.is_empty() {
                    break 'outer;
                }
                i -= 1;
            }
            curr_left_right[k] =
                map_array[next_key[0] as usize][next_key[1] as usize][next_key[2] as usize];
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
        assert_eq!("6", solve_puzzle_array(input));
    }
}
