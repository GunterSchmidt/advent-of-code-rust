pub mod part1;
pub mod part2;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_04.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

const FLOOR_SIZE: usize = 142;
const PAPER_ROLL: u8 = b'@';
// const SPACE: u8 = b'.';

/// Map of the floor with one byte extra on each side.
#[derive(Debug)]
pub struct Floor {
    map: [[u8; FLOOR_SIZE]; FLOOR_SIZE],
    size: usize,
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            map: [[0; FLOOR_SIZE]; FLOOR_SIZE],
            size: 0,
        }
    }
}

/// Parses the input str to the required data format.
/// Requires floor to be a square in size with max 140 chars.
pub fn parse_data(input: &str) -> Floor {
    let mut floor = Floor::default();

    // parse data as_bytes for performance reasons
    // look for line end
    let s = input.as_bytes().iter().position(|&c| c == b'\n').unwrap() + 1;
    let bytes_line_break = if input.as_bytes()[s - 2] == b'\r' {
        2
    } else {
        1
    };
    floor.size = s - bytes_line_break;

    // This copies the values fast, but they should be converted into 0 and 1 so the count
    // later works without using if statements.
    // for i in 0..floor.size {
    //     let line = &mut floor.map[i + 1][1..floor.size + 1];
    //     line.copy_from_slice(&input.as_bytes()[i * s..i * s + floor.size]);
    // }

    let mut row = 1;
    let mut col = 0;
    for &c in input.as_bytes() {
        col += 1;
        match c {
            // Set only paper roll, map is initialized with 0.
            PAPER_ROLL => floor.map[row][col] = 1,
            b'\n' => {
                row += 1;
                col = 0;
            }
            _ => {}
        }
    }

    floor
}
