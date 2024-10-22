pub mod part1_fast;
pub mod part1_pos_compare;
pub mod part2;
pub mod part2_no_hashmap;
pub mod parts_kotlin_converted;

// required for bench
pub const DAY_STR: &str = "day_03";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_03.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;
