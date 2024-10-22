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

1. Store number and symbol positions of line, keep data for 3 valid lines
2. Compare each number position check if a symbol is near by

*/

const EMPTY: u8 = b'.';
const POSITIONS_SIZE: usize = 20;
type NumberPositions = [NumberData; POSITIONS_SIZE];
type SymbolPositions = [Position; POSITIONS_SIZE];
type Position = u8;
type Value = u32;

#[derive(Default, Clone, Copy)]
struct LineData {
    /// create position array which stores pos_start, pos_end (inclusive) for max 20 numbers in a line
    number_positions: NumberPositions,
    n_number_positions: usize,
    symbol_positions: SymbolPositions,
    n_symbol_positions: usize,
}

impl LineData {
    /// fill next line and store position of numbers and symbols
    fn fill_work_idx_2(&mut self, b_line: &[u8]) {
        let num_positions_next = &mut self.number_positions;
        let mut n_num_positions_next = 0;
        let sym_positions_next = &mut self.symbol_positions;
        let mut n_sym_positions_next = 0;
        let mut number_started = false;
        let mut value: Value = 0;

        for (i, &byte) in b_line.iter().enumerate() {
            match byte {
                b'0'..=b'9' => {
                    if !number_started {
                        number_started = true;
                        num_positions_next[n_num_positions_next].pos_start = (i + 1) as Position;
                        n_num_positions_next += 1;
                        value = (byte - b'0') as Value;
                    } else {
                        value = value * 10 + (byte - b'0') as Value;
                    }
                    byte - b'0'
                }

                EMPTY => {
                    if number_started {
                        number_started = false;
                        // store end position (does not work for last element in line)
                        num_positions_next[n_num_positions_next - 1].pos_end = i as Position;
                        num_positions_next[n_num_positions_next - 1].value = value;
                    }
                    byte
                }

                // symbol
                _ => {
                    if number_started {
                        number_started = false;
                        // store end position (does not work for last element in line)
                        num_positions_next[n_num_positions_next - 1].pos_end = i as Position;
                        num_positions_next[n_num_positions_next - 1].value = value;
                    }
                    sym_positions_next[n_sym_positions_next] = (i + 1) as Position;
                    n_sym_positions_next += 1;
                    byte
                }
            };
        }
        // number at end of line?
        if number_started {
            num_positions_next[n_num_positions_next - 1].pos_end = b_line.len() as Position;
            num_positions_next[n_num_positions_next - 1].value = value;
        }

        self.n_number_positions = n_num_positions_next;
        self.n_symbol_positions = n_sym_positions_next;
    }
}

#[derive(Default, Clone, Copy)]
struct NumberData {
    pos_start: Position,
    /// end position of number (inclusive)
    pos_end: Position,
    value: Value,
}

/// Main function
/// Filter numbers which are adjacent to a symbol and return their sum.
pub fn solve_puzzle(input: &str) -> String {
    // For the check to work the next line must already be known,
    // so lines is always one ahead.

    // create work array of 3 lines
    // let mut work: Work = [[EMPTY; LINE_SIZE]; 3];
    let mut lines = [LineData::default(); 3];
    let mut result = 0;
    let mut input_lines = input.lines();
    // Init
    let b_line = input_lines.next().unwrap().as_bytes();
    lines[2].fill_work_idx_2(b_line);

    while let Some(next_line) = input_lines.next() {
        // shift work lines
        // TODO swap
        lines[0] = lines[1];
        lines[1] = lines[2];
        let b_line = next_line.as_bytes();
        lines[2].fill_work_idx_2(b_line);

        if lines[1].n_number_positions > 0 {
            result += check_adjacent(&lines);
        }
    }
    // last line
    if lines[2].n_number_positions > 0 {
        // shift work lines
        lines[0] = lines[1];
        lines[1] = lines[2];
        lines[2] = LineData::default();
        result += check_adjacent(&lines);
    }

    result.to_string()
}

/// checks each number for an ajacent symbol, moves symbol idx to avoid comparisons
fn check_adjacent(lines: &[LineData; 3]) -> Value {
    let mut result = 0;
    let positions_curr = &lines[1].number_positions;
    // check is adjacent curr line
    let mut first_symbol_idx = [0; 3];
    'numbers: for i_pos in 0..lines[1].n_number_positions {
        let number = &positions_curr[i_pos];
        let pos_start = number.pos_start - 1;
        let pos_end = number.pos_end + 1;
        // check is adjacent curr line
        for symbol_idx in first_symbol_idx[1]..lines[1].n_symbol_positions {
            let symbol_pos = lines[1].symbol_positions[symbol_idx];
            if symbol_pos == pos_start || symbol_pos == pos_end {
                // println!("{}", number.value);
                result += number.value;
                if symbol_pos < pos_end {
                    first_symbol_idx[1] += 1;
                }
                continue 'numbers;
            }
            if symbol_pos < pos_start {
                first_symbol_idx[1] += 1;
            }
        }

        // check is adjacent prev line
        for symbol_idx in first_symbol_idx[0]..lines[0].n_symbol_positions {
            let symbol_pos = lines[0].symbol_positions[symbol_idx];
            if symbol_pos >= pos_start && symbol_pos <= pos_end {
                // println!("{}", number.value);
                result += number.value;
                if symbol_pos < pos_end {
                    first_symbol_idx[0] += 1;
                }
                continue 'numbers;
            }
            if symbol_pos < pos_start {
                first_symbol_idx[0] += 1;
            }
        }

        // check is adjacent next line
        for symbol_idx in first_symbol_idx[2]..lines[2].n_symbol_positions {
            let symbol_pos = lines[2].symbol_positions[symbol_idx];
            if symbol_pos >= pos_start && symbol_pos <= pos_end {
                // println!("{}", number.value);
                result += number.value;
                if symbol_pos < pos_end {
                    first_symbol_idx[2] += 1;
                }
                continue 'numbers;
            }
            if symbol_pos < pos_start {
                first_symbol_idx[2] += 1;
            }
        }
    }

    result
}

/// checks for adjacent symbols, but always start with the first symbol, which is inefficient
#[allow(dead_code)]
fn check_adjacent_simple(lines: &[LineData; 3]) -> Value {
    let mut result = 0;
    let positions_curr = &lines[1].number_positions;
    // check is adjacent curr line
    'numbers: for i_pos in 0..lines[1].n_number_positions {
        let number = &positions_curr[i_pos];
        let pos_start = number.pos_start - 1;
        let pos_end = number.pos_end + 1;
        // check is adjacent curr line
        for symbol_idx in 0..lines[1].n_symbol_positions {
            let symbol_pos = lines[1].symbol_positions[symbol_idx];
            if symbol_pos == pos_start || symbol_pos == pos_end {
                // println!("{}", number.value);
                result += number.value;
                continue 'numbers;
            }
        }

        // check is adjacent prev line
        for symbol_idx in 0..lines[0].n_symbol_positions {
            let symbol_pos = lines[0].symbol_positions[symbol_idx];
            if symbol_pos >= pos_start && symbol_pos <= pos_end {
                // println!("{}", number.value);
                result += number.value;
                continue 'numbers;
            }
        }

        // check is adjacent next line
        for symbol_idx in 0..lines[2].n_symbol_positions {
            let symbol_pos = lines[2].symbol_positions[symbol_idx];
            if symbol_pos >= pos_start && symbol_pos <= pos_end {
                // println!("{}", number.value);
                result += number.value;
                continue 'numbers;
            }
        }
    }

    result
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
