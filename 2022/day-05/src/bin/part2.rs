use day_05::{part2::process, FILENAME_PART_2};
use aoc_file_reader::read_file;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_2);
    println!("{}", process(&input));
}
