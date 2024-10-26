/*!
# AoC 2023 Day 11 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/11>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Optimized coding for same algorithm as part 1.

*/

use smallvec::SmallVec;

pub type Position = u32;
pub type GalaxyPosition = SmallVec<[(Position, Position); 10]>;

const COLS: usize = 142;

pub fn solve_puzzle(input: &str) -> String {
    let galaxies = parse_universe(input, 1);
    // match universes and find shortest paths
    let mut sum_len_paths = 0;
    for i_first in 0..galaxies.0.len() {
        let first_row = galaxies.0[i_first];
        let first_col = galaxies.1[i_first];
        let sum = galaxies
            .0
            .iter()
            .enumerate()
            .skip(i_first)
            .map(|(i, &second_row)| {
                let second_col = galaxies.1[i];
                (first_row as i32 - second_row as i32).abs()
                    + (first_col as i32 - second_col as i32).abs()
            })
            .sum::<i32>() as u32;
        sum_len_paths += sum;
    }

    sum_len_paths.to_string()
}

pub fn parse_universe(input: &str, expansion_factor: Position) -> (Vec<u32>, Vec<u32>) {
    // Create list of universes with their row and column position.
    // This allows expansion of the universe by just adjusting the row and col positions.
    // Also the rest of the code only works on numbers, which is very fast. TODO

    // parse and store all galaxy positions
    let size = input.as_bytes().iter().position(|c| *c == b'\n').unwrap();
    let mut galaxy_positions: (Vec<Position>, Vec<Position>) =
        (Vec::with_capacity(size * 4), Vec::with_capacity(size * 4));
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                galaxy_positions.0.push(row as Position);
                galaxy_positions.1.push(col as Position);
            }
        }
    }

    // Expand rows
    // Loop over rows, find gaps and add to expansion to following rows.
    // This works, because rows come in ascending.
    // TODO smarter with vec, only one loop to add
    // let mut rows_unused = Vec::with_capacity(size / 10);
    let mut i_pos = galaxy_positions.0.len() - 1;
    let mut next_row = galaxy_positions.0[i_pos] - 1;
    while i_pos > 0 {
        i_pos -= 1;
        let row = galaxy_positions.0[i_pos];
        if row == next_row {
            next_row -= 1;
            if next_row == 0 {
                break;
            }
            continue;
        }
        while row < next_row {
            // rows_unused.push(next_row);
            next_row -= 1;
            for gp in galaxy_positions.0[i_pos + 1..].iter_mut() {
                *gp += expansion_factor;
            }
        }
        if row > next_row {
            continue;
        }
        next_row -= 1;
        if next_row == 0 {
            break;
        }
    }
    // one loop update is actually a tiny bit slower
    // now update within one loop
    // let start = galaxy_positions
    //     .0
    //     .iter()
    //     .position(|row| row > rows_unused.last().unwrap())
    //     .unwrap();
    // let mut expansions = 1;
    // let mut i_next = rows_unused.len() - 2;
    // let mut next = rows_unused[i_next];
    // for gp in galaxy_positions.0[start..].iter_mut() {
    //     while *gp > next {
    //         expansions += 1;
    //         if i_next > 0 {
    //             i_next -= 1;
    //             next = rows_unused[i_next];
    //         } else {
    //             next = u32::MAX;
    //         }
    //     }
    //     *gp += expansion_factor * expansions;
    // }

    // Expand columns
    // One could sort and dedup the col vector, but this are a lot of operations, which take slightly longer.
    let mut cols_used = [false; COLS];
    let mut cols_last = 0;
    for &col in galaxy_positions.1.iter() {
        cols_used[col as usize] = true;
        cols_last = cols_last.max(col);
    }

    for (col, _) in cols_used // [0..cols_last as usize + 1] // makes is slower
        .into_iter()
        .enumerate()
        .rev()
        .filter(|(i, c)| *i <= cols_last as usize && !(*c))
    {
        for gp in galaxy_positions.1.iter_mut() {
            if *gp as usize > col {
                *gp += expansion_factor;
            }
        }
    }

    // dbg!(&galaxy_positions);
    // let s: u32 = galaxy_positions.0.iter().sum::<u32>() + galaxy_positions.1.iter().sum::<u32>();
    // dbg!(s);
    // for p in galaxy_positions.0.iter() {
    //     print!("{p:3},")
    // }
    // println!();
    // for p in galaxy_positions.1.iter() {
    //     print!("{p:3},")
    // }
    galaxy_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", solve_puzzle(input));
    }
}
