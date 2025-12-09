/*!

# AoC 2025 Day 07 part 1
See the description of the puzzle at <https://adventofcode.com/2025/day/7>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

*/

use crate::{parse_data, RAY, SIZE, SPLIT};

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let data = parse_data(input);
    // dbg!(&data);

    // There must be a split below the S, this is hard coded.
    let mut cnt_splits = 1;
    let mut min = data.field[0].iter().position(|b| *b == b'S').unwrap() - 1;
    let mut max = min + 2;
    let mut rays = [0u8; SIZE];
    for i in 2..data.lines {
        for j in min + 2..max - 1 {
            if rays[j] == RAY && data.field[i][j] == SPLIT {
                cnt_splits += 1;
                rays[j] = SPLIT;
                rays[j - 1] = RAY;
                rays[j + 1] = RAY;
            }
        }
        if data.field[i][min] == SPLIT {
            cnt_splits += 1;
            rays[min] = SPLIT;
            min -= 1;
            rays[min] = RAY;
            rays[min + 2] = RAY;
        }
        if data.field[i][max] == SPLIT {
            cnt_splits += 1;
            rays[max] = SPLIT;
            max += 1;
            rays[max] = RAY;
            rays[max - 2] = RAY;
        }
    }

    cnt_splits.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!("21", solve_puzzle(input));
    }
}
