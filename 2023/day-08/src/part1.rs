/*!
# AoC 2023 Day 8 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/8>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

I use hashbrown::HashMap which is significantly faster than HashMap.
I am using LEFT = 0 and RIGHT = 1 instead of an enum, because this allows direct access as index.

*/

use hashbrown::HashMap;

pub const START_KEY: u32 = address_to_number("AAA");
pub const END_KEY: u32 = address_to_number("ZZZ");

/// This uses a simple for loop
pub fn solve_puzzle(input: &str) -> String {
    // The HashMap is used to store the left-right and top-bottom rules.
    // The three chars are converted to a number, so working will be fast.

    let (directions, map, _) = parse_input(input);

    let mut steps: u32 = 1;
    let mut current = map.get(&START_KEY).unwrap();
    // repeat direction until ZZZ is reached
    'outer: loop {
        for d in &directions {
            let key = current[*d];
            if key == END_KEY {
                break 'outer;
            } else {
                current = map.get(&current[*d]).unwrap();
                steps += 1;
            }
        }
    }

    // dbg!(result_sum);

    steps.to_string()
}

/// Parse into hashbrown HashMap
pub fn parse_input(input: &str) -> (Vec<usize>, HashMap<u32, [u32; 2]>, Vec<u32>) {
    let mut lines = input.lines();
    let mut map: HashMap<u32, [u32; 2]> = HashMap::with_capacity(800);
    let mut start_keys = Vec::with_capacity(10);

    // first line is left-right rule
    let left_right = lines.next().unwrap();
    // direction is a vector for left-right, left = true
    let directions: Vec<usize> = left_right
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    lines.next();

    while let Some(line) = lines.next() {
        let key = address_to_number(&line[0..3]);
        let left = address_to_number(&line[7..10]);
        let right = address_to_number(&line[12..15]);
        if key as u8 == b'A' {
            start_keys.push(key);
        }

        map.insert(key, [left, right]);
    }

    (directions, map, start_keys)
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
        assert_eq!("2", solve_puzzle(input));
    }
}
