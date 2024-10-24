use aoc_file_reader::read_file;
use day_06::{part2_loop, part2_math, FILENAME_PART_2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_2);
    println!("{}", part2_loop::solve_puzzle(&input));
    println!("{}", part2_math::solve_puzzle(&input));
}
