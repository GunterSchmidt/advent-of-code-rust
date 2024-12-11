pub mod puzzle;

// required for bench
pub const DAY_STR: &str = "day_11";
pub const WARM_UP_TIME_MS: u64 = 500;
pub const MEASUREMENT_TIME_MS: u64 = 2000;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_11.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

/// allow print to console and file output
pub const WITH_OUTPUT_PRINT: bool = false;
/// allow file output
pub const WITH_OUTPUT_FILE: bool = false;

type DataInt = u64;

#[inline]
fn num_digits(number: u64) -> u32 {
    return match number {
        0..10 => 1,
        10..100 => 2,
        100..1000 => 3,
        1000..10000 => 4,
        10000..100000 => 5,
        100000..1000000 => 6,
        1000000..10000000 => 7,
        10000000..100000000 => 8,
        _ => number.ilog10() + 1,
    };
}

/// Parses data as_bytes for performance reasons.
pub fn parse_data(input: &str) -> Vec<DataInt> {
    let mut data = Vec::with_capacity(10);

    let mut n = 0;
    let mut is_number = false;
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => {
                n = n * 10 + (c - b'0') as DataInt;
                is_number = true
            }
            b' ' | b'\n' => {
                if is_number {
                    data.push(n);
                    n = 0;
                    is_number = false;
                }
            }
            _ => (),
        }
    }
    // last element, line may not have a line ending
    if is_number {
        data.push(n);
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(3, num_digits(999));
        assert_eq!(4, num_digits(1000));
        assert_eq!(8, num_digits(10000000));
        assert_eq!(9, num_digits(199999999));
    }
}
