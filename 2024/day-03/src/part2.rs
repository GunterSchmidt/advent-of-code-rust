/*!

# AoC 2024 Day 03 part 2
See the description of the puzzle at <https://adventofcode.com/2024/day/03>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
Parse a text.

Here two solutions are presented, the short one using regex and a manual parser.
The manual parser is 150 times (!) faster than regex.

*/

use regex::Regex;

/// Part 2: Parsing manually is very fast, runs in 0.0145 ms which is 75 times faster than regex!
pub fn solve_puzzle(input: &str) -> String {
    let mut result = 0;
    let data = input.as_bytes();
    let mut do_multiply = true;
    let mut i = 0;
    while i < data.len() - 8 {
        if do_multiply {
            if data[i] == b'm' && data[i + 1] == b'u' && data[i + 2] == b'l' && data[i + 3] == b'('
            {
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
            } else if data[i] == b'd'
                && data[i + 1] == b'o'
                && data[i + 2] == b'n'
                && data[i + 3] == b'\''
                && data[i + 4] == b't'
                && data[i + 5] == b'('
                && data[i + 6] == b')'
            {
                do_multiply = false;
            }
        } else if data[i] == b'd'
            && data[i + 1] == b'o'
            && data[i + 2] == b'('
            && data[i + 3] == b')'
        {
            do_multiply = true;
        }
        i += 1;
    }

    result.to_string()
}

/// Part 2 using regex. Concise but slow, takes about two milliseconds.
pub fn solve_puzzle_regex(input: &str) -> String {
    // regex: mul\(\d+\,\d+\)
    // let data = input.lines().map(|line| {}).collect::<Vec<_>>();
    // for line in input.lines() {}
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)|(do\(\))|(don't\(\))").unwrap();

    let mut do_multiply = true;
    let result: u32 = re
        .captures_iter(input)
        .map(|c| {
            let mut multi = 0;
            if do_multiply {
                // test first letter of matches
                match c[0].as_bytes()[0] {
                    b'm' => {
                        let a = c[1].parse::<u32>().unwrap();
                        let b = c[2].parse::<u32>().unwrap();
                        multi = a * b
                    }
                    b'd' => do_multiply = &c[0] == "do()",
                    _ => panic!("Should not happen"),
                }
            } else {
                do_multiply = &c[0] == "do()"
            }
            // dbg!(&c, do_multiply, multi);
            multi
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", solve_puzzle(input));
    }
}
