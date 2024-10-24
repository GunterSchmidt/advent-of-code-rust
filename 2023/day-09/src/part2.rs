/*!
# AoC 2023 Day 9 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/9>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

The program is looping over each row separately. This is a small amount of data.
Instead of using an array SmallVec is used, which speeds up the logic by 50%.
The atoi crate could have been used, but handimplementing the logic saves 10%.

*/

use smallvec::SmallVec;

pub type NumPuzzle = i32;
pub type NumSmallVec = SmallVec<[i32; 21]>;
// pub type NumSmallVec = Vec<i32>;

pub fn solve_puzzle(input: &str) -> String {
    // create line parser
    input
        .lines()
        .map(|line| {
            let numbers = line
                .as_bytes()
                .split(|c| *c == b' ')
                // .map(|n| atoi::atoi::<isize>(n).unwrap())
                .map(|n| atoi(n))
                // .collect::<Vec<_>>();
                .collect::<NumSmallVec>();
            // store only last two numbers of delta, should be enough to calculate
            get_next(numbers) as isize
        })
        .sum::<isize>()
        .to_string()
}

/// Calculates the next number in the order
/// Seems inefficient as the numbers have a specific structure
#[inline]
fn get_next(mut numbers: NumSmallVec) -> NumPuzzle {
    let last_top_level = numbers.len() - 1;
    let mut delta_1 = numbers[last_top_level] - numbers[last_top_level - 1];
    // let mut number_stack: Vec<Vec<NumPuzzle>> = Vec::with_capacity(15);
    // let mut number_stack: Vec<Vec<i32>> = Vec::new();
    let mut number_stack = SmallVec::<[NumSmallVec; 18]>::new();
    while delta_1 != 0 {
        // next level
        let last = numbers.len() - 1;
        let delta_2 = numbers[last - 1] - numbers[last - 2];
        if delta_1 == delta_2 {
            // assume final delta found
            let mut prev = numbers[0] - delta_1;
            for stack in number_stack.iter().rev() {
                // next += stack.last().unwrap();
                prev = stack[0] - prev;
            }
            return prev;
        }
        // calc deltas
        // Each level, one less number gets calculated
        let numbers_next = numbers
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<NumSmallVec>();
        number_stack.push(numbers);
        numbers = numbers_next;
        delta_1 = delta_1 - delta_2;
    }
    panic!("logic error")
}

#[inline]
fn atoi(num: &[u8]) -> NumPuzzle {
    let mut int = 0;
    if num[0] == b'-' {
        for i in 1..num.len() {
            if num[i].is_ascii_digit() {
                int = int * 10 + (num[i] - b'0') as NumPuzzle;
            }
        }
        return -int;
    }
    for i in 0..num.len() {
        if num[i].is_ascii_digit() {
            int = int * 10 + (num[i] - b'0') as NumPuzzle;
        }
    }
    int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", solve_puzzle(input));
    }
}
