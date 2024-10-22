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

1. Store asterix positions of line
2. Store copy of line in array, keep 3 valid lines
3. Compare each asterix position with line array and check if a number is near by
4. Store numbers for a * in a HashMap to identify cases where two numbers link to a *.

*/

use hashbrown::HashMap;

const LINE_SIZE: usize = 142;
const EMPTY: u8 = b'.';
type Work = [[u8; LINE_SIZE]; 3];
type AsterixPositions = [Vec<usize>; 3];
const ASTERIX: u8 = b'*';

/// Main function
/// Filter pairs of numbers which are adjacent to a * symbol and return the total sum of their products.
pub fn solve_puzzle(input: &str) -> String {
    // For the check to work the next line must already be known,
    // so lines is always one ahead.

    // create work array of 3 lines
    let mut work: Work = [[EMPTY; LINE_SIZE]; 3];
    // For each work line store the positions of all * symbols
    let mut asterix_positions: AsterixPositions = [Vec::new(), Vec::new(), Vec::new()];
    let mut result = 0;
    let mut input_lines = input.lines();
    // Init
    let b_line = input_lines.next().unwrap().as_bytes();
    fill_work_idx_2(&mut work, &mut asterix_positions, b_line);
    let mut symbol_map: HashMap<(usize, usize), usize> = hashbrown::HashMap::new();

    let mut i_line = 0;
    while let Some(next_line) = input_lines.next() {
        i_line += 1;
        // shift work lines
        asterix_positions.swap(0, 1);
        asterix_positions.swap(1, 2);
        work[0] = work[1];
        work[1] = work[2];
        let b_line = next_line.as_bytes();
        fill_work_idx_2(&mut work, &mut asterix_positions, b_line);

        if asterix_positions[1].len() > 0 {
            let ret = find_numbers_adjacent_asterix(&work, &mut asterix_positions);
            for set in ret {
                // println!("L{s_line}-{}={}", set.1, set.2);
                if let Some(num_value) = symbol_map.get(&(i_line, set.0)) {
                    // already a value for this number, so this is a gear.
                    // Once the value is used it is not required anymore, keeping the map small.
                    result += num_value * set.1;
                } else {
                    symbol_map.insert((i_line, set.0), set.1);
                }
            }
        }
    }
    // last line
    if asterix_positions[2].len() > 0 {
        i_line += 1;
        // shift work lines
        asterix_positions.swap(0, 1);
        asterix_positions.swap(1, 2);
        asterix_positions[2] = Vec::new();
        work[0] = work[1];
        work[1] = work[2];
        let ret = find_numbers_adjacent_asterix(&mut work, &asterix_positions);
        for set in ret {
            if let Some(num_value) = symbol_map.get(&(i_line, set.0)) {
                // already a value for this number, so this is a gear.
                // Once the value is used it is not required anymore, keeping the map small.
                result += num_value * set.1;
            } else {
                symbol_map.insert((i_line, set.0), set.1);
            }
        }
    }

    result.to_string()
}

/// fill next work line and store position of symbols, here only asterix
#[inline]
fn fill_work_idx_2(work: &mut Work, asterix_positions: &mut AsterixPositions, b_line: &[u8]) {
    asterix_positions[2].clear();
    for (i, &byte) in b_line.iter().enumerate() {
        let b = match byte {
            b'0'..=b'9' => byte - b'0',
            ASTERIX => {
                asterix_positions[2].push(i + 1);
                byte
            }
            _ => byte,
        };
        work[2][i + 1] = b;
    }
}

/// Checks if a numbers of that line are adjacent to the * Symbol.
/// Returns a vec of found numbers (Symbol work line, pos, number)
#[inline]
fn find_numbers_adjacent_asterix(
    work: &Work,
    asterix_positions: &AsterixPositions,
) -> Vec<(usize, usize)> {
    let mut result_numbers = Vec::new();
    // check is adjacent curr line
    for &pos in asterix_positions[1].iter() {
        if work[1][pos - 1] < 10 {
            // create number while going backwards
            let mut p = pos - 1;
            let mut n = work[1][p] as usize;
            p -= 1;
            let mut factor = 10;
            while work[1][p] < 10 {
                n += work[1][p] as usize * factor;
                factor *= 10;
                p -= 1;
            }
            result_numbers.push((pos, n));
        };
        if work[1][pos + 1] < 10 {
            // create number while going forwards
            let mut p = pos + 1;
            let mut n = work[1][p] as usize;
            p += 1;
            while work[1][p] < 10 {
                n = n * 10 + work[1][p] as usize;
                p += 1;
            }
            result_numbers.push((pos, n));
        };

        // check is adjacent prev line
        // these are the three around the star
        for p in pos - 1..pos + 2 {
            if work[0][p] < 10 {
                // search for start of number
                let mut q = p - 1;
                while work[0][q] < 10 {
                    q -= 1;
                }
                q += 1;
                let mut n = work[0][q] as usize;
                q += 1;
                while work[0][q] < 10 {
                    n = n * 10 + work[0][q] as usize;
                    q += 1;
                }
                result_numbers.push((pos, n));
                if q == pos {
                    // need to check if two numbers surround this star
                    continue;
                }
                break;
            }
        }
        // check is adjacent next line
        // these are the three around the star
        for p in pos - 1..pos + 2 {
            if work[2][p] < 10 {
                // search for start of number
                let mut q = p - 1;
                while work[2][q] < 10 {
                    q -= 1;
                }
                q += 1;
                let mut n = work[2][q] as usize;
                q += 1;
                while work[2][q] < 10 {
                    n = n * 10 + work[2][q] as usize;
                    q += 1;
                }
                result_numbers.push((pos, n));
                if q == pos {
                    // need to check if two numbers surround this star
                    continue;
                }
                break;
            }
        }
    }

    result_numbers
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
        assert_eq!("467835", solve_puzzle(input));
    }

    #[test]
    fn test_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$*.....
.664.598..";
        assert_eq!("413417", solve_puzzle(input));
    }
}
