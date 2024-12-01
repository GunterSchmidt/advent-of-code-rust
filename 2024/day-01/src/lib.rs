pub mod part1;
pub mod part2;

// required for bench
pub const DAY_STR: &str = "day_01";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_01.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

/// allow print to console and file output
pub const WITH_OUTPUT_PRINT: bool = false;
/// allow file output
pub const WITH_OUTPUT_FILE: bool = false;

type Num = i32;

fn parse_data(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut data_first = Vec::with_capacity(1000);
    let mut data_second = Vec::with_capacity(1000);

    // parse data as_bytes for performance reasons
    let mut n = 0;
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as Num,
            b' ' => {
                if n > 0 {
                    data_first.push(n);
                    n = 0;
                }
            }
            b'\n' => {
                data_second.push(n);
                n = 0;
            }
            _ => (),
        }
    }
    if n > 0 {
        data_second.push(n);
    }
    assert_eq!(data_first.len(), data_second.len());
    data_first.sort_unstable();
    data_second.sort_unstable();

    (data_first, data_second)
}
