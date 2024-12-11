use aoc_file_reader::read_file;
use day_11::{puzzle, FILENAME_PART_1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = read_file(FILENAME_PART_1);
    let (part_1, part_2) = puzzle::solve_both(&input);
    println!("part 1: {part_1}\npart 2: {part_2}");
}
