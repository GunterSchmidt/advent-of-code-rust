use aoc_file_reader::read_file;
use day_05::{parts_both::solve_puzzle_array, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    let result = solve_puzzle_array(&input);
    println!("part 1: {}", result.0);
    println!("part 2: {}", result.1);
}
