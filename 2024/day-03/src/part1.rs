/*!

# AoC 2024 Day 03 part 1
See the description of the puzzle at <https://adventofcode.com/2024/day/03>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Parse a text.

Here two solutions are presented, the short one using regex and a manual parser.
The manual parser is 75 times (!) faster than regex.

*/

use regex::Regex;

/// Part 1: Parsing manually is very fast, runs in 0.0145 ms which is 75 times faster than regex!
pub fn solve_puzzle(input: &str) -> String {
    let mut result = 0;
    let data = input.as_bytes();
    let mut i = 0;
    while i < data.len() - 8 {
        if data[i] == b'm' && data[i + 1] == b'u' && data[i + 2] == b'l' && data[i + 3] == b'(' {
            i += 4;
            let mut n = 0;
            let mut first = 0;
            loop {
                match data[i] {
                    b'0'..=b'9' => n = n * 10 + (data[i] - b'0') as u32,
                    b',' => {
                        first = n;
                        n = 0;
                    }
                    b')' => {
                        result += first * n;
                        break;
                    }
                    // This can happen if the mul is not closing correctly
                    _ => break,
                }
                i += 1;
            }
        }
        i += 1;
    }

    result.to_string()
}

/// Part 1 using regex. Concise but slow, takes about one millisecond.
pub fn solve_puzzle_regex(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

    let result: u32 = re
        .captures_iter(input)
        .map(|c| {
            let a = c[1].parse::<u32>().unwrap();
            let b = c[2].parse::<u32>().unwrap();
            a * b
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", solve_puzzle(input));
    }
}
