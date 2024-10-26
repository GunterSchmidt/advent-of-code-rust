use aoc_file_reader::read_file;
use day_11::{part2_v1::solve_puzzle, part2_v2, FILENAME_PART_2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_2);
    println!("{}", solve_puzzle(&input));
    println!("{}", part2_v2::solve_puzzle(&input));
}
