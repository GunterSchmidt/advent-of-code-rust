/*!
# AoC 2022 Day 1 part 2
See the description of the puzzle at <https://adventofcode.com/2022/day/1>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>
---

Using fold to solve the puzzle.

*/

use crate::part1::atoi;

pub fn solve_puzzle(input: &str) -> String {
    // Adding end marker so that the last value is used.
    const END: [&[u8]; 1] = [&[0b0]];
    let (_, top_3) = input.as_bytes().split(|b| *b == b'\n').chain(END).fold(
        (0, [0; 3]),
        |(elf_calories, top_3), line| {
            if line.is_empty() || line[0] == 0 {
                // reset elf counter
                if elf_calories < top_3[2] {
                    (0, top_3)
                } else {
                    // complicated but fast top 3 implementation
                    let mut top_3_new = top_3.clone();
                    let mut i = 0;
                    while i < 3 {
                        if elf_calories > top_3[i] {
                            top_3_new[i] = elf_calories;
                            // shift elements to keep array sorted
                            let mut j = 2;
                            while j > i {
                                top_3_new[j] = top_3[j - 1];
                                j -= 1;
                            }
                            return (0, top_3_new);
                        } else {
                            top_3_new[i] = top_3[i];
                        }
                        i += 1;
                    }
                    panic!("logic error")
                }
            } else {
                // add calory for current elf
                // let elf_calories = elf_calories + line.parse::<u32>().unwrap();
                let elf_calories = elf_calories + atoi(line);
                (elf_calories, top_3)
            }
        },
    );
    top_3.iter().sum::<usize>().to_string()
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
        assert_eq!("45000", solve_puzzle(input));
    }
}
