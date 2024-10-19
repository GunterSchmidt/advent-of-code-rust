/*!
# AoC 2022 Day 1 part 2
See the description of the puzzle at <https://adventofcode.com/2022/day/1>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>
---

The top 3 list is created on the stack with an array for better performance.

*/
use crate::part1::{Calory, Elf};

type Top3 = [Calory; 3];

pub fn process(input: &str) -> String {
    let elfes = Elf::parse_input(input);

    let top_3 = top_3_v2(&elfes);
    let sum = top_3.iter().sum::<u32>();
    sum.to_string()
}

#[allow(dead_code)]
/// Simple version
fn top_3_v1(elfes: &[Elf]) -> Top3 {
    // create top 3 list
    // array based, as this is faster than a vector
    let mut top_3 = [0; 3];
    for elf in elfes.iter() {
        let cal = elf.sum_calories();
        let mut i = 0;
        while i < 3 {
            if cal > top_3[i] {
                // shift elements to keep array sorted
                let mut j = 2;
                while j > i {
                    top_3[j] = top_3[j - 1];
                    j -= 1;
                }
                top_3[i] = cal;
                break;
            }
            i += 1;
        }
    }

    top_3
}

#[allow(dead_code)]
/// Slightly improved version checking on last element first to avoid search from top.
/// Also using local memory instead of array access. Very small improvement.
/// Backwards search would be possible but would require more logic to find insert position.
fn top_3_v2(elfes: &[Elf]) -> Top3 {
    // create top 3 list
    // array based, as this is faster than a vector
    let mut top_3 = [0; 3];
    let mut min = 0;
    for elf in elfes.iter() {
        let cal = elf.sum_calories();
        if cal < min {
            continue;
        }
        let mut i = 0;
        while i < 3 {
            if cal > top_3[i] {
                // shift elements to keep array sorted
                let mut j = 2;
                while j > i {
                    top_3[j] = top_3[j - 1];
                    j -= 1;
                }
                top_3[i] = cal;
                min = top_3[2];
                break;
            }
            i += 1;
        }
    }

    top_3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
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
        assert_eq!("45000", process(input));
    }
}
