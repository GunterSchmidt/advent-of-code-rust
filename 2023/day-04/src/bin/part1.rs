use aoc_file_reader::read_file;
use day_04::{part1_array::solve_puzzle, FILENAME_PART_1};
// use day_04::{part1_str::solve_puzzle, FILENAME_PART_1};
// use day_04::{part1_vec:solve_puzzle, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", solve_puzzle(&input));
}
