/*!
# AoC 2023 Day 11 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/11>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This solution is not very fast.
Issue 1: Parsing from lines (instead of just the bytes) into Vec<Vec>> (instead of a single vec) is fairly slow.
Issue 2: The Expansion of the universe requires multiple loops.

*/

use smallvec::SmallVec;

pub type Position = u32;
pub type GalaxyPosition = SmallVec<[(Position, Position); 10]>;

const COLS: usize = 142;

pub fn solve_puzzle(input: &str) -> String {
    let galaxies = parse_universe(input, 1);
    // match universes and find shortest paths
    let mut sum_len_paths = 0;
    for (i_first, (first_row, first_col)) in galaxies.iter().enumerate() {
        let sum = galaxies
            .iter()
            .skip(i_first)
            .map(|(second_row, second_col)| {
                (*first_row as i32 - *second_row as i32).abs()
                    + (*first_col as i32 - *second_col as i32).abs()
            })
            .sum::<i32>() as u32;
        sum_len_paths += sum;
    }

    sum_len_paths.to_string()
}

pub fn parse_universe(input: &str, expansion_factor: Position) -> Vec<(u32, u32)> {
    // Create list of universes with their row and column position.
    // This allows expansion of the universe by just adjusting the row and col positions.
    // Also the rest of the code only works on numbers, which is very fast.

    let mut universe_rows: Vec<GalaxyPosition> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    b'#' => Some((row as Position, col as Position)),
                    b'.' => None,
                    _ => panic!("Invalid character: {}", c),
                })
                .collect::<GalaxyPosition>()
        })
        .collect();

    // expand rows and collect used columns
    // let mut cols_used = vec![false; universe_rows.len()];
    let mut cols_used = [false; COLS];
    let mut cols_last = 0;
    for row in (0..universe_rows.len()).rev() {
        let universe_row = &mut universe_rows[row];
        for u in universe_row.iter() {
            cols_used[u.1 as usize] = true;
            cols_last = cols_last.max(u.1);
        }
        if universe_row.is_empty() && row < universe_rows.len() - 1 {
            for ur in universe_rows[row + 1..].iter_mut() {
                for u in ur {
                    u.0 += expansion_factor;
                }
            }
        }
    }

    // expand columns
    // flatten universes to single vector as later the index of the vector is used to calculate the sum
    let mut galaxies: Vec<(Position, Position)> = universe_rows.into_iter().flatten().collect();
    for (col, _) in cols_used //[0..cols_last as usize + 1] // makes is slower
        .into_iter()
        .enumerate()
        .rev()
        .filter(|(_, c)| !(*c))
    {
        for ur in galaxies.iter_mut() {
            if ur.1 as usize > col {
                ur.1 += expansion_factor;
            }
        }
    }

    // dbg!(&galaxies);
    // let s: u32 = galaxies.iter().map(|it| it.0 + it.1).sum::<u32>();
    // dbg!(s);
    // for p in galaxies.iter() {
    //     print!("{:3},", p.0)
    // }
    // println!();
    // for p in galaxies.iter() {
    //     print!("{:3},", p.1)
    // }

    galaxies
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
