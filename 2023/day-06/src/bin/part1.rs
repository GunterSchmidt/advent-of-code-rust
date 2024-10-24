use aoc_file_reader::read_file;
// use day_06::{part1_loop::solve_puzzle, FILENAME_PART_1};
use day_06::{part1_loop, part1_math, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", part1_loop::solve_puzzle(&input));
    println!("{}", part1_math::solve_puzzle(&input));
}
