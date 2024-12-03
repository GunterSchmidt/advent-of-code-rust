use aoc_file_reader::read_file;
use day_03::{
    part2::{solve_puzzle, solve_puzzle_regex},
    FILENAME_PART_2,
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_2);
    println!("{}", solve_puzzle(&input));
    println!("{}", solve_puzzle_regex(&input));
}
