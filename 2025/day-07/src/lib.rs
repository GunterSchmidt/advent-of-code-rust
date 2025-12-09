pub mod part1;
pub mod part2;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_07.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

const SIZE: usize = 145;
pub type DataArr = [[u8; SIZE]; SIZE];
pub const SPLIT: u8 = b'^';
pub const RAY: u8 = b'|';

pub struct RayField {
    pub field: DataArr,
    pub len: usize,
    pub lines: usize,
}

impl Default for RayField {
    fn default() -> Self {
        Self {
            field: [[0u8; SIZE]; SIZE],
            len: 0,
            lines: 0,
        }
    }
}

/// Parses the input str to the required data format.
/// Requires floor to be a square in size with max 140 chars.
pub fn parse_data(input: &str) -> RayField {
    let mut ray_field = RayField::default();

    // Parse data as_bytes for performance reasons.
    // Only read every second line as the blank lines deliver no information.
    // Assume identical line length, no error handling.
    let in_bytes = input.as_bytes();
    let len = in_bytes.iter().position(|c| *c == b'\n').unwrap() + 1;
    ray_field.len = len - 1;
    ray_field.lines = in_bytes.len() / len / 2;
    for i in 0..ray_field.lines {
        let i2 = i * 2;
        ray_field.field[i][0..len - 1].copy_from_slice(&in_bytes[i2 * len..i2 * len + len - 1]);
    }

    ray_field
}
