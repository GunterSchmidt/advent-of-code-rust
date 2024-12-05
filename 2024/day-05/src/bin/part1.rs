use aoc_file_reader::read_file;
use day_05::{
    part1::{solve_part1_array, solve_part1_hashmap},
    FILENAME_PART_1,
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    println!("{}", solve_part1_array(&input));
    println!("{}", solve_part1_hashmap(&input));
}
