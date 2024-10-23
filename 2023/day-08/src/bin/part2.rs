use aoc_file_reader::read_file;
use day_08::{
    part2_v1::solve_puzzle_v1, part2_v2::solve_puzzle_v2, part2_v3_array::solve_puzzle_array,
    FILENAME_PART_2,
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_2);
    println!("{}", solve_puzzle_v1(&input));
    println!("{}", solve_puzzle_v2(&input));
    println!("{}", solve_puzzle_array(&input));
}
