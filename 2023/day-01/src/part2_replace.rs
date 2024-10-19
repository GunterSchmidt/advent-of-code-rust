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
Using rust map with sum to get the result.
The trick is to replace all number words with the digit,
but keeping first and last character for overlapping numbers.
This is very slow since a lot of string operations are performed and
shows that string operations really need to be kept to a minimum, especially
writing/changing strings.

*/

// Find first and last digit (also as word) in each String, return sum.
pub fn solve_puzzle(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            // replace all words with numbers, but keep first and last character for overlapping numbers
            // this is exremely slow
            // better approach would be to get the first and last digit first, remember the index of the first and last digit
            // and search only before and after for words
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            // quick filter on all characters
            // performance: after first is found it would be faster to iterate backwards till first from back is found
            let mut digits = line.chars().filter_map(|character| character.to_digit(10));
            // first number, must be present
            let first = digits.next().expect("Should be a number");
            // last number, may be absent
            match digits.last() {
                Some(v) => first * 10 + v,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();

    result.to_string()
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
