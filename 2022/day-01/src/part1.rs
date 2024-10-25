/*!
# AoC 2022 Day 1 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/1>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

This is a simple sum function.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let (_, max) =
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .fold((0, 0), |(elf_calories, max), line| {
                if line.is_empty() {
                    // reset elf counter
                    (0, max)
                } else {
                    // add calory for current elf
                    // let elf_calories = elf_calories + line.parse::<u32>().unwrap();
                    let elf_calories = elf_calories + atoi(line);
                    (elf_calories, elf_calories.max(max))
                }
            });
    max.to_string()
}

#[inline]
pub fn atoi(num: &[u8]) -> usize {
    let mut int = 0;
    for i in 0..num.len() {
        if num[i].is_ascii_digit() {
            int = int * 10 + (num[i] - b'0') as usize;
        }
    }
    int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!("24000", solve_puzzle(input));
    }
}
