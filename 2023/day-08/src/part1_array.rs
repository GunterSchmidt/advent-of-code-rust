/*!
# AoC 2023 Day 8 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/8>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This uses an array instead of a HashMap.

Create an array for all 3 letter combinations.
There are 26 letters (and 2 numbers for test). This can fit in a 5 bit window each,
making a total of 15 bits. As such the key can be single u16 (instead of a 3 dimensional array).

Additionally the parser was optimized.

*/

pub const START_KEY: usize = 0;
pub const END_KEY: u16 = 0b11001_11001_11001;

/// This uses a simple for loop
pub fn solve_puzzle_array(input: &str) -> String {
    // The HashMap is used to store the left-right and top-bottom rules.
    // The three chars are converted to a number, so working will be fast.

    let (directions, map_array) = parse_input_array(input);
    // return 0.to_string();
    let mut steps: u32 = 1;
    let mut current = map_array[START_KEY];
    // repeat direction until ZZZ is reached
    'outer: loop {
        for d in &directions {
            let key = current[*d];
            // let key = unsafe { *current.get_unchecked(*d) };
            if key == END_KEY {
                break 'outer;
            } else {
                current = map_array[current[*d] as usize];
                steps += 1;
            }
        }
    }

    // dbg!(result_sum);

    steps.to_string()
}

/// Parse into array
pub fn parse_input_array(input: &str) -> (Vec<usize>, [[u16; 2]; 27483]) {
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
    let mut map_array = [[0; 2]; 0b11010_11010_11010 + 1];
    while b_input.get(line_start + jump - 2).is_some() {
        let line = &b_input[line_start..];
        line_start += jump;

        fn u8_key_to_u16(a: u8, b: u8, c: u8) -> u16 {
            ((a - b'A') as u16) << 10 | ((b - b'A') as u16) << 5 | (c - b'A') as u16
        }

        let key = u8_key_to_u16(line[0], line[1], line[2]);
        let left = u8_key_to_u16(line[7], line[8], line[9]);
        let right = u8_key_to_u16(line[12], line[13], line[14]);

        map_array[key as usize] = [left, right];
    }

    (directions, map_array)
}

/// Converts [u8;3] into u32
pub const fn address_to_number(address: &str) -> u32 {
    let b = address.as_bytes();
    let n = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
    n
}

pub fn number_to_address(n: u32) -> String {
    let b = n.to_be_bytes();
    core::str::from_utf8(&b[1..]).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", solve_puzzle_array(input));
    }
}
