/*!
# AoC 2023 Day 1 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/1>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Find first and last digit in each String, return sum.
This uses a for loop over all lines.
There are two searches, one from start and one from end of the string.
String is treated as bytes as input is expected to be ASCII.
Performance is almost three times as fast as part1_compact.rs.
Using rust build in functionaliy and yes, zero cost abstraction is true.

*/

type Result = u32;

// Find first and last digit in each String, return sum.
pub fn solve_puzzle(input: &str) -> String {
    let mut result_sum: Result = 0;
    for line in input.lines() {
        // assume ASCII
        // find first digit in line
        let value;
        if let Some(first_digit) = line.bytes().find(|&byte| matches!(byte, b'1'..=b'9')) {
            value = (first_digit - b'0') * 10;
        } else {
            panic!("no digit found in line: {line}");
        }
        // working with .chars() is slower than .bytes
        // if let Some(first_digit) = line.chars().find(|char | char.is_ascii_digit()) {
        //     value = (first_digit as u8 - b'0') * 10;
        // } else {
        //     panic!("no digit found in line: {line}");
        // }

        // find last digit in line
        // since digit was found, no more validation
        let last_digit = line
            .bytes()
            .rev()
            .find(|&byte| matches!(byte, b'1'..=b'9'))
            .unwrap()
            - b'0';

        result_sum += (value + last_digit) as Result;
    }

    result_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", solve_puzzle(input));
    }
}
