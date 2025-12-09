/*!

# AoC 2025 Day 07 part 2
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

    // remember most left and most right ray
    let mut min = data.field[0].iter().position(|b| *b == b'S').unwrap() - 1;
    let mut max = min + 2;
    // keep track of all ray positions
    let mut rays = [0u8; SIZE];
    // count timelines per column
    let mut timelines = [0u64; SIZE];
    timelines[min] = 1;
    timelines[max] = 1;
    for i in 2..data.lines {
        for j in min + 2..max - 1 {
            if rays[j] == RAY && data.field[i][j] == SPLIT {
                let cur = timelines[j];
                timelines[j] = 0;
                rays[j] = SPLIT;
                rays[j - 1] = RAY;
                rays[j + 1] = RAY;
                timelines[j - 1] += cur;
                timelines[j + 1] += cur;
            }
        }
        if data.field[i][min] == SPLIT {
            let cur = timelines[min];
            timelines[min] = 0;
            rays[min] = SPLIT;
            min -= 1;
            rays[min] = RAY;
            rays[min + 2] = RAY;
            timelines[min] += cur;
            timelines[min + 2] += cur;
        }
        if data.field[i][max] == SPLIT {
            let cur = timelines[max];
            timelines[max] = 0;
            rays[max] = SPLIT;
            max += 1;
            rays[max] = RAY;
            rays[max - 2] = RAY;
            timelines[max] += cur;
            timelines[max - 2] += cur;
        }
    }
    let cnt_timelines: u64 = timelines.iter().sum();

    cnt_timelines.to_string()
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
        assert_eq!("40", solve_puzzle(input));
    }
}
