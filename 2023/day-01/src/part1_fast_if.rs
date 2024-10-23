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

*/

/// Find first and last digit in each String, return sum.
pub fn solve_puzzle(input: &str) -> String {
    let mut result_sum: u32 = 0;
    for line in input.lines() {
        let mut value: u8 = 0;
        // find first digit in line
        for byte in line.bytes() {
            // assume ASCII
            // if matches!(byte, b'1'..=b'9') {
            if byte <= b'9' && byte >= b'1' {
                value = (byte - b'0') * 10;
                break;
            }
        }
        if value == 0 {
            panic!("no digit found in line: {line}");
        }

        // find last digit in line
        // since first digit was found, no more validation required
        for byte in line.bytes().rev() {
            // assume ASCII
            // if matches!(byte, b'1'..=b'9') {
            if byte <= b'9' && byte >= b'1' {
                value += byte - b'0';
                break;
            }
        }

        result_sum += value as u32;
    }

    result_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", solve_puzzle(input));
    }
}
