pub mod part1;
pub mod part2;

// required for criterion bench
pub const DAY_STR: &str = "day_06";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 3000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_06.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;
