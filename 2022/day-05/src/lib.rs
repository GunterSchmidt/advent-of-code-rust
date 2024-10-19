pub mod part1;
pub mod part2;
pub mod stack;

// required for bench
pub const DAY_STR: &str = "day_05";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_05.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

/// allow print to console and file output
pub const WITH_OUTPUT_PRINT: bool = false;
