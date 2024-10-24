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

Create an array for all 3 letter combinations.
There are 26 letters (and 2 numbers for test). This can fit in a 5 bit window each,
making a total of 15 bits. As such the key can be single u16 (instead of a 3 dimensional array).

Additionally the parser was optimized.

*/

use crate::part2_v1::least_common_denominator_with_multiplication;

pub fn solve_puzzle_array_16_bit_key(input: &str) -> String {
    const END_KEY: u16 = (b'Z' - b'A') as u16;

    // parse data
    let (directions, map_array, start_keys) = parse_input_array_v2(input);
    // find pathes ends
    let key_count = start_keys.len();
    let mut found_z_steps: Vec<u64> = vec![0; key_count];

    let mut curr_left_right = start_keys
        .iter()
        .map(|k| map_array[*k as usize])
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
            if next_key & 0b00000_00000_11111 == END_KEY {
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
            curr_left_right[k] = map_array[next_key as usize];
            i += 1;
        }
        result_steps += 1;
    }
    let combined_result = least_common_denominator_with_multiplication(&found_z_steps);

    combined_result.to_string()
}

/// Parse into array
pub fn parse_input_array_v2(input: &str) -> (Vec<usize>, [[u16; 2]; 27483], Vec<u16>) {
    let b_input = input.as_bytes();
    let mut line_end = b_input.iter().position(|&c| c == b'\n').unwrap();
    let jump;
    let mut line_start = if b_input[line_end - 1] == b'\r' {
        line_end -= 1;
        jump = 18;
        line_end + 4
    } else {
        jump = 17;
        line_end + 2
    };

    // first line is left-right rule
    // direction is a vector for left-right, left = true
    let directions: Vec<usize> = b_input[0..line_end]
        .iter()
        .map(|c| if *c == b'L' { 0 } else { 1 })
        .collect();

    // create array for all 3 letter combinations
    // There are 26 letters (and 2 numbers for test). This can fit in a 5 bit window each,
    // making a total of 15 bits. As such the key can be single u16 (instead of a 3 dimensional array).
    // Instead of 26^3 (17576) 8-bit keys now 15^3 (3375) 16-bit keys are required; this does not make
    // the array smaller.
    // First case requires 26^3 * 2 * 3 u8 = 105,456 byte.
    // Here the highest number is 26 which is 11010 in binary. 11010_11010_11010 is the highest combination,
    // which is 27483 * 2 * u16 = 109,932 byte.
    let mut start_keys = Vec::with_capacity(10);
    let mut map_array = [[0; 2]; 0b11010_11010_11010 + 1];
    while b_input.get(line_start + jump - 2).is_some() {
        let line = &b_input[line_start..];
        line_start += jump;

        #[inline(always)]
        fn u8_to_u16(c: u8) -> u16 {
            if c >= b'A' {
                c as u16 - b'A' as u16
            } else {
                c as u16 - 44
            }
        }

        #[inline(always)]
        fn u8_key_to_u16(a: u8, b: u8, c: u8) -> u16 {
            u8_to_u16(a) << 10 | u8_to_u16(b) << 5 | u8_to_u16(c)
        }

        let key = u8_key_to_u16(line[0], line[1], line[2]);
        let left = u8_key_to_u16(line[7], line[8], line[9]);
        let right = u8_key_to_u16(line[12], line[13], line[14]);

        // collect start keys
        if key & 0b00000_00000_11111 == 0 {
            start_keys.push(key);
        }
        map_array[key as usize] = [left, right];
    }

    (directions, map_array, start_keys)
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
        assert_eq!("6", solve_puzzle_array_16_bit_key(input));
    }
}
