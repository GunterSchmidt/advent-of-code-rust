/*!
# AoC 2023 Day 1 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/1>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Find first and last digit (also as word) in each String, return sum.
Here the given numbers are searched for, each separately.
This is very slow since a lot of string search operations are performed.

*/

// min word length for later addition
const LEN_MIN: usize = 3;
const NUMBER_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// Find first and last digit (also as word) in each String, return sum.
pub fn solve_puzzle(input: &str) -> String {
    let mut result_sum: usize = 0;

    let mut has_digit = false;
    let lines = input.lines();
    for line in lines {
        let len_line = line.len();
        let mut value_first: usize = 0;
        let mut pos_first = len_line;

        // find first digit in line and store position
        if let Some((pos, char)) = line.char_indices().find(|(_, char)| char.is_ascii_digit()) {
            // store value, can be temporary
            value_first = char.to_digit(10).unwrap() as usize * 10;
            pos_first = pos;
            has_digit = true;
        }

        // search for first spelled out number
        // only until the best known position
        // min_pos now holds the position of the first found digit or the len of the line
        // a spelled out number must be found before min_pos
        if pos_first >= LEN_MIN {
            let mut number_idx = usize::MAX;
            for (i, el) in NUMBER_NAMES.iter().enumerate() {
                let f = &line[..pos_first].find(el);
                if f.is_some() && pos_first > f.unwrap() {
                    pos_first = f.unwrap() + el.len() - 1;
                    number_idx = i;
                    if f.unwrap() < LEN_MIN {
                        break;
                    }
                }
            }

            if number_idx < usize::MAX {
                value_first = (number_idx + 1) * 10
            }
        }

        // find last digit in line
        let mut value_last = usize::MAX;
        let mut pos_last = pos_first;
        // no need to search again if no digit exists
        if has_digit {
            for (pos, char) in line.chars().rev().enumerate() {
                if char.is_ascii_digit() {
                    value_last = char.to_digit(10).unwrap() as usize;
                    pos_last = len_line - pos - 1;
                    break;
                }
            }
        }

        // search for last spelled out number
        // only from best known position
        let mut number_idx = usize::MAX;
        if pos_last + LEN_MIN < len_line {
            let mut l = &line[pos_last + 1..];
            for (i, el) in NUMBER_NAMES.iter().enumerate() {
                let f = l.rfind(el);
                if f.is_some() && pos_last < f.unwrap() + pos_last + 1 {
                    pos_last = f.unwrap() + pos_last + el.len() - 1; // -1 for overlapping;
                    number_idx = i;
                    // any more possible?
                    if pos_last + LEN_MIN >= len_line {
                        break;
                    }
                    l = &line[pos_last + 1..];
                }
            }
        }
        if number_idx < usize::MAX {
            value_first += number_idx + 1;
        } else {
            value_first += value_last;
        }
        result_sum += value_first;
        // println!("{value}: {line}");
    }

    result_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", solve_puzzle(input));
    }
}
