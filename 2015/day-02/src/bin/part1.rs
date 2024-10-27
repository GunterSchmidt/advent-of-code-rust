use aoc_file_reader::read_file;
use day_02::{
    part1::{solve_puzzle_fast, solve_puzzle_rusty, solve_puzzle_rusty_atoi},
    FILENAME_PART_1,
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", solve_puzzle_fast(&input));
    println!("{}", solve_puzzle_rusty(&input));
    println!("{}", solve_puzzle_rusty_atoi(&input));
}
