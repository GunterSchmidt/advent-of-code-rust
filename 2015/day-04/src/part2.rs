/*!

# AoC 2015 Day 4 part 2
See the description of the puzzle at <https://adventofcode.com/2015/day/4>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Find an md5 Hash with a given start sequence.

Since it runs very long also a threaded version was implemented without external crates.

*/

use crate::{
    part1::{scan_range, solve_puzzle_threads, StopZeros, MAX},
    WITH_OUTPUT_PRINT,
};
use std::thread;

// CPUs reported by system or this number, whatever is lower
const MAX_THREADS: usize = 20;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let min_md5 = scan_range(input, 0, MAX as u32, StopZeros::Six).unwrap();
    if WITH_OUTPUT_PRINT {
        println!("solve_puzzle: {min_md5}");
    }

    min_md5.number.to_string()
}

/// The main function for this puzzle as threaded version.
pub fn solve_puzzle_threaded(input: &str) -> String {
    let num_cpus: usize = thread::available_parallelism().unwrap().into();
    let num_cpus = num_cpus.min(MAX_THREADS);

    let min_md5 = solve_puzzle_threads(input, StopZeros::Six, num_cpus as u8);
    if WITH_OUTPUT_PRINT {
        println!("solve_puzzle_threaded: {min_md5}");
    }

    min_md5.number.to_string()
}
