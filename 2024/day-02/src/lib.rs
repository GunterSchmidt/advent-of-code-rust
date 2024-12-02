pub mod part1;
pub mod part2;

// required for bench
pub const DAY_STR: &str = "day_02";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_02.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

/// allow print to console and file output
pub const WITH_OUTPUT_PRINT: bool = false;
/// allow file output
pub const WITH_OUTPUT_FILE: bool = false;

type Num = i32; // requires change of first bit
const FIRST_BIT: Num = 0b1000000_00000000_00000000_00000000;
const MAX_ELEMENTS: usize = 9;

fn parse_data(input: &str) -> Vec<Vec<Num>> {
    let mut data = Vec::with_capacity(1000);

    // parse data as_bytes for performance reasons
    let mut data_line = Vec::with_capacity(9);
    let mut n = 0;
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as Num,
            b' ' | b'\n' => {
                data_line.push(n);
                n = 0;
                if c == b'\n' {
                    data.push(data_line);
                    data_line = Vec::with_capacity(9);
                }
            }
            _ => (),
        }
    }
    if n > 0 {
        data_line.push(n);
        data.push(data_line);
    }

    data
}
