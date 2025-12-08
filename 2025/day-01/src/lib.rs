use std::fmt::Display;

pub mod part1;
pub mod part2;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_01.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

type Num = i16;

// This only holds the direction. The alternative to also store the clicks is less clear in code and slower.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    clicks: Num,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.direction == Direction::Left {
            write!(f, "L")?;
        } else {
            write!(f, "R")?;
        }
        write!(f, "{}", self.clicks)
    }
}

fn parse_data(input: &str) -> Vec<Rotation> {
    let mut data = Vec::with_capacity(1000);

    // parse data as_bytes for performance reasons
    let mut n = 0;
    let mut r = Rotation {
        direction: Direction::Left,
        clicks: 0,
    };
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as Num,
            b'L' => r.direction = Direction::Left,
            b'R' => r.direction = Direction::Right,
            b'\n' => {
                r.clicks = n;
                data.push(r);
                n = 0;
            }
            _ => (),
        }
    }
    if n > 0 {
        data.push(r);
    }

    data
}
