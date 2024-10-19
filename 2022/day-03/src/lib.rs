pub mod lib_day;
pub mod part1_array;
pub mod part1_loop;
pub mod part2_array;
pub mod part2_loop;

// required for bench
pub const DAY_STR: &str = "day_03";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2500;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_03.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;
