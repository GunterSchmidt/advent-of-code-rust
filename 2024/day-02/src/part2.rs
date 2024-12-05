/*!

# AoC 2024 Day 02 part 2
See the description of the puzzle at <https://adventofcode.com/2024/day/02>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Filter list for ascending/descending steps 1 to 3 with one error allowed.

There are suprising many constellations which is difficult to check. As such
once an error occurs, the two constellations with one item removed are checked.

The code uses an array which is much faster than a vector.

*/

use crate::{Num, FIRST_BIT, MAX_ELEMENTS};

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    // parse single line
    // since only one line an array can be used
    let mut data_line = [0; MAX_ELEMENTS];
    let mut safe = 0;
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
                    // if evaluate_line(&data_line, pos) == 0 {
                    //     println!("unsafe {:?}", &data_line[0..pos]);
                    // } else {
                    //     println!("OK {:?}", &data_line[0..pos]);
                    // }
                    safe += evaluate_line(&data_line, pos);
                    pos = 0;
                }
            }
            _ => (),
        }
    }
    if n > 0 {
        safe += evaluate_line(&data_line, pos);
    }

    safe.to_string()
}

// #[inline(always)] // makes it slower, even other functions not using this(!)
fn evaluate_line(line: &[Num; MAX_ELEMENTS], len: usize) -> usize {
    // There are 3 possible errors:
    // - duplicate element
    // - distance too large
    // - change of direction
    // The problem is to identify the wrong element.
    // This could be wrong if first element is totally off.
    let direction_bit = (line[len - 1] - line[0]) & FIRST_BIT;
    let mut last_n = line[0];
    for p in 1..len {
        let delta = line[p] - last_n;
        // no need to check last element if no error so far
        if p + 1 == len {
            return 1;
        }

        // check same direction by comparing first bit
        if delta == 0 || delta.abs() > 3 || (direction_bit != delta & FIRST_BIT) {
            // possibly unsafe, check possible combinations
            if p > 1 {
                let delta2 = line[p] - line[p - 2];
                if delta2 != 0 && delta2.abs() <= 3 && (direction_bit == delta2 & FIRST_BIT) {
                    // skip p-1 could work
                    let mut line2 = *line;
                    line2.copy_within(p..len, p - 1);
                    if evaluate_line_no_error(&line2, len - 1, direction_bit) == 1 {
                        return 1;
                    }
                }
            } else {
                // skip first element
                let mut line2 = *line;
                line2.copy_within(1..len, 0);
                if evaluate_line_no_error(&line2, len - 1, direction_bit) == 1 {
                    return 1;
                }
            }
            let delta2 = line[p + 1] - line[p - 1];
            if delta2 != 0 && delta2.abs() <= 3 && (direction_bit == delta2 & FIRST_BIT) {
                // skip p could work
                let mut line2 = *line;
                line2.copy_within(p + 1..len, p);
                return evaluate_line_no_error(&line2, len - 1, direction_bit);
            }
            return 0;
        } else {
            last_n = line[p]
        }
    }
    0
}

fn evaluate_line_no_error(line: &[Num; MAX_ELEMENTS], len: usize, direction_bit: Num) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "34 31 32 35 36 39
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("5", solve_puzzle(input));
    }
}
