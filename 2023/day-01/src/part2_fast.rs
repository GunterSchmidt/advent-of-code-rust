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
Here the given numbers are searched for with match operation.
The trick is to match each letter with the first letters of the number word.
This allows very fast comparison and also ensures any number word is found as first word.
This is extremely fast, but requires a lot of code.
Performance is almost 4 times faster then with search and 25 times faster then the
replacement version.
This shows that string operations really need to be kept to a minimum, especially
writing/changing strings.

*/

// min word length to reduce search operations
const MIN_NUMBER_NAME_LENGTH: usize = 3;

pub fn solve_puzzle(input: &str) -> String {
    let mut result_sum: usize = 0;

    let mut has_digit = false;
    let lines = input.lines();
    for line in lines {
        let len_line = line.len();
        let mut value: usize = 0;
        let mut pos_first_end = len_line;

        // find first digit in line and store position
        if let Some((pos, first_digit)) = line
            .bytes()
            .enumerate()
            .find(|(_, byte)| matches!(*byte, b'1'..=b'9'))
        {
            value = (first_digit - b'0') as usize * 10;
            pos_first_end = pos;
            has_digit = true;
        }

        // search for first spelled out number
        // only until the best known position
        // min_pos now holds the position of the first found digit or the len of the line
        // a spelled out number must be found before min_pos
        if pos_first_end >= MIN_NUMBER_NAME_LENGTH {
            // Optimize speed by checking for first letter of number.
            // This reduces the amount of search operations significantly
            // as not all numbers need to be checked first
            // and also eliminates the issue with overlapping numbers.
            for (i, byte) in line[..=pos_first_end - MIN_NUMBER_NAME_LENGTH]
                .bytes()
                .enumerate()
            {
                match byte {
                    b'o' => {
                        // one
                        if line[i + 1..].starts_with("ne") {
                            value = 10;
                            pos_first_end = i + 2;
                            break;
                        }
                    }
                    b't' => {
                        // two, three
                        if line[i + 1..].starts_with("wo") {
                            value = 20;
                            pos_first_end = i + 2;
                            break;
                        }
                        if line[i + 1..pos_first_end].starts_with("hree") {
                            value = 30;
                            pos_first_end = i + 4;
                            break;
                        }
                    }
                    b'f' => {
                        // four, five
                        if i + 3 < pos_first_end {
                            if line[i + 1..].starts_with("our") {
                                value = 40;
                                pos_first_end = i + 3;
                                break;
                            }
                            if line[i + 1..].starts_with("ive") {
                                value = 50;
                                pos_first_end = i + 3;
                                break;
                            }
                        }
                    }
                    b's' => {
                        // six, seven
                        if line[i + 1..].starts_with("ix") {
                            value = 60;
                            pos_first_end = i + 2;
                            break;
                        }
                        if line[i + 1..pos_first_end].starts_with("even") {
                            value = 70;
                            pos_first_end = i + 4;
                            break;
                        }
                    }
                    b'e' => {
                        // eight, five letters
                        if i + 4 < pos_first_end && line[i + 1..].starts_with("ight") {
                            value = 80;
                            pos_first_end = i + 4;
                            break;
                        }
                    }
                    b'n' => {
                        // nine, four letters
                        if i + 3 < pos_first_end && line[i + 1..pos_first_end].starts_with("ine") {
                            value = 90;
                            pos_first_end = i + 3;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        // find last digit in line
        let mut value_last = usize::MAX;
        let mut pos_last = pos_first_end;
        // no need to search again if no digit exists
        if has_digit {
            if let Some((pos, last_digit)) = line
                .bytes()
                .rev()
                .enumerate()
                .find(|(_, byte)| matches!(*byte, b'1'..=b'9'))
            {
                value_last = (last_digit - b'0') as usize;
                pos_last = len_line - pos - 1;
            }
        }

        if pos_last + MIN_NUMBER_NAME_LENGTH < len_line {
            // Optimize speed by checking for last letter of number.
            // This reduces the amount of search operations significantly
            // as not all numbers need to be checked first
            // and also eliminates the issue with overlapping numbers.
            // let len_remain = len_line - pos_last - 1;
            for (i, byte) in line[pos_last + MIN_NUMBER_NAME_LENGTH..]
                .bytes()
                .rev()
                .enumerate()
            {
                match byte {
                    b'e' => {
                        // one, three, five, nine
                        // second match on third last letter as all these numbers have that letter
                        match line[len_line - 3 - i..].bytes().next().unwrap() {
                            b'o' => {
                                if line[len_line - 3 - i..].starts_with("on") {
                                    value_last = 1;
                                    break;
                                }
                            }
                            b'r' => {
                                if len_line - i >= 5 && line[len_line - 5 - i..].starts_with("thre")
                                {
                                    value_last = 3;
                                    break;
                                }
                            }
                            b'i' => {
                                if len_line - i >= 4 {
                                    if line[len_line - 4 - i..].starts_with("fiv") {
                                        value_last = 5;
                                        break;
                                    }
                                    if line[len_line - 4 - i..].starts_with("nin") {
                                        value_last = 9;
                                        break;
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    b'o' => {
                        // two
                        if line[len_line - 3 - i..].starts_with("tw") {
                            value_last = 2;
                            break;
                        }
                    }
                    b'r' => {
                        // four
                        if len_line - i >= 4 && line[len_line - 4 - i..].starts_with("fou") {
                            value_last = 4;
                            break;
                        }
                    }
                    b'x' => {
                        // six
                        if line[len_line - 3 - i..].starts_with("si") {
                            value_last = 6;
                            break;
                        }
                    }
                    b'n' => {
                        // seven
                        if len_line - i >= 5 && line[len_line - 5 - i..].starts_with("seve") {
                            value_last = 7;
                            break;
                        }
                    }
                    b't' => {
                        // eight, five letters
                        if len_line - i >= 5 && line[len_line - 5 - i..].starts_with("eigh") {
                            value_last = 8;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }
        result_sum += value + value_last;
        // println!("{value}: {line}");
    }

    result_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
