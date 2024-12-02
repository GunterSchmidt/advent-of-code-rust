/*!

# AoC 2024 Day 02 part 1
See the description of the puzzle at <https://adventofcode.com/2024/day/02>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Filter list for ascending/descending steps 1 to 3.

*/

use crate::{parse_data, Num, FIRST_BIT, MAX_ELEMENTS};

pub fn solve_puzzle(input: &str) -> String {
    // parse single line
    // since only one line an array can be used
    let mut data_line = [0; MAX_ELEMENTS];
    let mut safe_count = 0;
    let mut n = 0;
    let data = input.as_bytes();
    let last = data.len() - 1;
    let mut pos = 0;
    for (i, &c) in data.iter().enumerate() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as Num,
            b' ' | b'\n' => {
                data_line[pos] = n;
                pos += 1;
                n = 0;
                if c == b'\n' || i == last {
                    // evaluate line
                    safe_count += evaluate_line(&data_line, pos);
                    pos = 0;
                }
            }
            _ => (),
        }
    }
    if n > 0 {
        safe_count += evaluate_line(&data_line, pos);
    }

    safe_count.to_string()
}

// #[inline(always)] // makes it slower, even other functions not using this(!)
fn evaluate_line(line: &[Num; MAX_ELEMENTS], len: usize) -> usize {
    // This could be wrong if first element is totally off.
    let direction_bit = (line[len - 1] - line[0]) & FIRST_BIT;
    for p in 1..len {
        let delta = line[p] - line[p - 1];
        // check same direction by comparing first bit
        if delta == 0 || delta.abs() > 3 || (direction_bit != delta & FIRST_BIT) {
            // unsafe
            return 0;
        } else if p == len - 1 {
            return 1;
        }
    }
    0
}

// reading all data first triples the time
pub fn solve_puzzle_parse_full(input: &str) -> String {
    let data = parse_data(input);
    let mut safe = 0;
    for line in data {
        let mut last_n = 0;
        let mut direction = 0;
        for (i, &n) in line.iter().enumerate() {
            if direction == 0 {
                // first and second number
                if last_n > 0 {
                    direction = n - last_n;
                    if direction == 0 || direction.abs() > 3 {
                        break;
                    }
                }
                last_n = n;
            } else {
                let delta = n - last_n;
                // check same direction by comparing first bit
                if delta == 0 || delta.abs() > 3 || (direction & FIRST_BIT != delta & FIRST_BIT) {
                    // unsafe, so go to next
                    break;
                } else if i == line.len() - 1 {
                    safe += 1;
                } else {
                    last_n = n;
                }
            }
        }
    }

    safe.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", solve_puzzle(input));
    }
}
