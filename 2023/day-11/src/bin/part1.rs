use aoc_file_reader::read_file;
use day_11::{part1_timvisee, part1_v1::solve_puzzle, part1_v2, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", solve_puzzle(&input));
    println!("{}", part1_v2::solve_puzzle(&input));
    println!("{}", part1_timvisee::solve_puzzle(&input));
}
