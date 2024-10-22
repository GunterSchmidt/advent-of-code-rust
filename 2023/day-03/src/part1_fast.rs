/*!
# AoC 2023 Day 3 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/3>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Filter numbers which are adjacent to a symbol.
Input is ASCII, so String will be parsed as_bytes().
For comparison the previous, current and next line are required, this is
not much data and can be put in an array.
To avoid border checks the data will be extended by a frame of one char,
so that +1 and -1 always yield a valid element.

1. Store number positions of line
2. Store copy of line in array, keep 3 valid lines
3. Compare each number position with line array and check if a symbol is near by

*/

const EMPTY: u8 = b'.';
const POSITIONS_SIZE: usize = 20;
type Positions = [NumberPos; POSITIONS_SIZE];
type Position = usize;
type Value = usize;

#[derive(Default, Clone, Copy)]
struct NumberPos {
    pos_start: Position,
    /// position exclusive
    pos_end: Position,
    value: Value,
}

/// Main function
/// Filter numbers which are adjacent to a symbol and return their sum.
pub fn solve_puzzle(input: &str) -> String {
    // For the check to work the next line must already be known,
    // so lines is always one ahead.

    let mut result = 0;
    // read all lines in a vector for access to next line
    let b_lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    // create position array which stores pos_start, pos_end (exclusive), value for max 20 numbers in a line
    let mut positions_curr = [NumberPos::default(); POSITIONS_SIZE];
    for (i_line, &b_line) in b_lines.iter().enumerate() {
        let n_positions_curr = fill_positions(b_line, &mut positions_curr);
        result += check_adjacent_line(&b_lines, i_line, n_positions_curr, &positions_curr);
    }

    result.to_string()
}

/// Fill next work line and store position of numbers.
// #[inline]
fn fill_positions(b_line: &[u8], positions_next: &mut Positions) -> usize {
    let mut n_positions_next = 0;
    let mut number_started = false;
    let mut value = 0;
    for (i, &byte) in b_line.iter().enumerate() {
        if byte.is_ascii_digit() {
            if !number_started {
                number_started = true;
                positions_next[n_positions_next].pos_start = i as Position;
                n_positions_next += 1;
                value = (byte - b'0') as Value;
            } else {
                value = value * 10 + (byte - b'0') as Value;
            }
        } else {
            if number_started {
                number_started = false;
                // store end position (does not work for last element in line)
                positions_next[n_positions_next - 1].pos_end = i as Position;
                positions_next[n_positions_next - 1].value = value;
            }
        };
    }
    // number at end of line?
    if number_started {
        positions_next[n_positions_next - 1].pos_end = b_line.len() as Position;
        positions_next[n_positions_next - 1].value = value;
    }

    n_positions_next
}

/// checks if a symbol is next to a number and adds the numbers of this line
// #[inline]
fn check_adjacent_line(
    b_lines: &[&[u8]],
    line_no: usize,
    n_positions_curr: usize,
    positions_curr: &Positions,
) -> usize {
    if n_positions_curr == 0 {
        return 0;
    }
    let mut result = 0;

    // check is adjacent first num, which can be at start of line
    // dividing this save 8%
    'first: loop {
        let pos_start = positions_curr[0].pos_start;
        let pos_end = positions_curr[0].pos_end;
        // check is adjacent curr line
        if (pos_start > 0 && b_lines[line_no][pos_start - 1] != EMPTY)
            || b_lines[line_no][pos_end] != EMPTY
        {
            // println!("{n}");
            result += positions_curr[0].value;
            break;
        };

        // check is adjacent prev line
        let last: usize = b_lines[0].len() - 1;
        let pos_start = pos_start.max(1);
        let pos_end = pos_end.min(last);
        if line_no > 0 {
            for p in pos_start - 1..pos_end + 1 {
                let byte = b_lines[line_no - 1][p];
                if byte != EMPTY && byte.is_ascii_punctuation() {
                    // println!("{n}");
                    result += positions_curr[0].value;
                    break 'first;
                }
            }
        }

        // check is adjacent next line
        if line_no < last {
            for p in pos_start - 1..pos_end + 1 {
                let byte = b_lines[line_no + 1][p];
                if byte != EMPTY && byte.is_ascii_punctuation() {
                    // if byte != EMPTY && byte > 9 {
                    // println!("{n}");
                    result += positions_curr[0].value;
                    break 'first;
                }
            }
        }
        break;
    }

    'positions: for n in 1..n_positions_curr - 1 {
        let pos_start = positions_curr[n].pos_start;
        let pos_end = positions_curr[n].pos_end;
        // check is adjacent curr line
        if b_lines[line_no][pos_start - 1] != EMPTY || b_lines[line_no][pos_end] != EMPTY {
            // println!("{n}");
            result += positions_curr[n].value;
            continue 'positions;
        };

        // check is adjacent prev line
        if line_no > 0 {
            for p in pos_start - 1..pos_end + 1 {
                let byte = b_lines[line_no - 1][p];
                if byte != EMPTY && byte.is_ascii_punctuation() {
                    // println!("{n}");
                    result += positions_curr[n].value;
                    continue 'positions;
                }
            }
        }
        // check is adjacent next line
        if line_no < b_lines.len() - 1 {
            for p in pos_start - 1..pos_end + 1 {
                let byte = b_lines[line_no + 1][p];
                if byte != EMPTY && byte.is_ascii_punctuation() {
                    // if byte != EMPTY && byte > 9 {
                    // println!("{n}");
                    result += positions_curr[n].value;
                    continue 'positions;
                }
            }
        }
    }

    // check is adjacent last num, which can be at end of line
    let n = n_positions_curr - 1;
    let pos_start = positions_curr[n].pos_start;
    let pos_end = positions_curr[n].pos_end;
    // check is adjacent curr line
    let last: usize = b_lines[0].len() - 1;
    if b_lines[line_no][pos_start - 1] != EMPTY
        || (pos_end <= last && b_lines[line_no][pos_end] != EMPTY)
    {
        // println!("{n}");
        result += positions_curr[n].value;
        return result as usize;
    };

    // check is adjacent prev line
    let pos_start = pos_start.max(1);
    let pos_end = pos_end.min(last);
    if line_no > 0 {
        for p in pos_start - 1..pos_end + 1 {
            let byte = b_lines[line_no - 1][p];
            if byte != EMPTY && byte.is_ascii_punctuation() {
                // println!("{n}");
                result += positions_curr[n].value;
                return result as usize;
            }
        }
    }

    // check is adjacent next line
    if line_no < last {
        for p in pos_start - 1..pos_end + 1 {
            let byte = b_lines[line_no + 1][p];
            if byte != EMPTY && byte.is_ascii_punctuation() {
                // if byte != EMPTY && byte > 9 {
                // println!("{n}");
                result += positions_curr[n].value;
            }
        }
    }

    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", solve_puzzle(input));
    }
}
