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
Algorithm uses a simple for loop over all lines.
It first creates an array of all digits which then is used to get first and last digits.
Performance drawbacks:
* create an additional small array
* loop over all chars of line instead of searching from start and then from end

In total 3x slower than the fast variants.

Idea credit to HyperNeutrino:
<https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day01p1.py>

*/

type Result = u32;

/// Find first and last digit in each String, return sum.
pub fn solve_puzzle(input: &str) -> String {
    let mut result_sum: Result = 0;
    for line in input.lines() {
        // create a vector of digits only
        let digits = line
            .chars()
            // filter only digits
            .filter(|char| char.is_ascii_digit())
            .map(|char| char as u8 - b'0')
            .collect::<Vec<_>>();

        // add first and last digit of vector to the result
        result_sum += (digits[0] * 10 + digits[digits.len() - 1]) as Result;
        // println!("{line}: {digits:?} = total {result_sum}");
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
