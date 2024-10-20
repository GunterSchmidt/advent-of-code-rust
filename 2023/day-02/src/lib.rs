pub mod part1_fast;
pub mod part1_str;
pub mod part2_fast;
pub mod part2_str;

// required for bench
pub const DAY_STR: &str = "day_02";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_02.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;
