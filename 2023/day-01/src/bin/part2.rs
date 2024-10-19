use aoc_file_reader::read_file;
// use day_01::{part2_fast::solve_puzzle, FILENAME_PART_2};
// use day_01::{part2_replace::solve_puzzle, FILENAME_PART_2};
use day_01::{part2_search::solve_puzzle, FILENAME_PART_2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_2);
    println!("{}", solve_puzzle(&input));
}
