use aoc_file_reader::read_file;
use day_01::{part1_fast_if::solve_puzzle, FILENAME_PART_1};
// use day_01::{part1_compact::solve_puzzle, FILENAME_PART_1};
// use day_01::{part1_fast_rusty::solve_puzzle, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", solve_puzzle(&input));
}
