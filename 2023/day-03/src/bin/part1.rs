use aoc_file_reader::read_file;
// use day_03::parts_kotlin_converted::solve_puzzle_1;
// use day_03::{part1_pos_compare::solve_puzzle, FILENAME_PART_1};
use day_03::{part1_fast::solve_puzzle, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", solve_puzzle(&input));
}
