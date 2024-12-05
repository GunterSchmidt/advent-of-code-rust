pub mod part1;
pub mod part2;
pub mod parts_both;

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
/// allow file output
pub const WITH_OUTPUT_FILE: bool = false;

use hashbrown::{HashMap, HashSet};
const ARRAY_SIZE: usize = 100;
type DataType = u16;

/// parses the file and return a HashMap with HashSets included and the pages to produce as vec
pub fn parse_data_hashmap(
    input: &str,
) -> (HashMap<DataType, HashSet<DataType>>, Vec<Vec<DataType>>) {
    const PAGES_CAPACITY: usize = 30;
    let mut map: HashMap<DataType, HashSet<DataType>> = HashMap::with_capacity(50);

    // parse data as_bytes for performance reasons
    // page ordering rules
    let mut n = 0;
    let mut key = 0;
    let mut iter = input.as_bytes().iter();
    for &c in &mut iter {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as DataType,
            b'|' => {
                key = n;
                n = 0;
            }
            b'\n' => {
                if n == 0 {
                    break;
                }
                if let Some(set) = map.get_mut(&key) {
                    set.insert(n);
                } else {
                    let mut set = HashSet::with_capacity(80);
                    set.insert(n);
                    map.insert(key, set);
                }
                n = 0;
            }
            _ => (),
        }
    }

    // pages to produce
    let mut pages_to_produce: Vec<Vec<DataType>> = Vec::with_capacity(210);
    let mut pages = Vec::with_capacity(PAGES_CAPACITY);
    let mut n = 0;
    for &c in &mut iter {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as DataType,
            b',' => {
                pages.push(n);
                n = 0;
            }
            b'\n' => {
                if n == 0 {
                    break;
                }
                pages.push(n);
                pages_to_produce.push(pages);
                pages = Vec::with_capacity(PAGES_CAPACITY);
                n = 0;
            }
            _ => (),
        }
    }

    // repeat for last entry if not ending with \n
    if n > 0 {
        pages.push(n);
        pages_to_produce.push(pages);
    }

    (map, pages_to_produce)
}

/// parses the file and returns an Array 100x100 where the value is true for allowed page followings,
/// e.g. 47|53 -> map[47][53] = true
/// And also the pages to produce as vec.
pub fn parse_data_array(input: &str) -> ([[bool; ARRAY_SIZE]; ARRAY_SIZE], Vec<Vec<DataType>>) {
    const PAGES_CAPACITY: usize = 30;

    // parse data as_bytes for performance reasons
    // page ordering rules
    let mut map = [[false; ARRAY_SIZE]; ARRAY_SIZE];
    let mut n = 0;
    let mut key = 0;
    let mut iter = input.as_bytes().iter();
    for &c in &mut iter {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as DataType,
            b'|' => {
                key = n;
                n = 0;
            }
            b'\n' => {
                if n == 0 {
                    break;
                }
                map[key as usize][n as usize] = true;
                n = 0;
            }
            _ => (),
        }
    }

    // pages to produce
    let mut pages_to_produce: Vec<Vec<DataType>> = Vec::with_capacity(210);
    let mut pages = Vec::with_capacity(PAGES_CAPACITY);
    let mut n = 0;
    for &c in &mut iter {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as DataType,
            b',' => {
                pages.push(n);
                n = 0;
            }
            b'\n' => {
                if n == 0 {
                    break;
                }
                pages.push(n);
                pages_to_produce.push(pages);
                pages = Vec::with_capacity(PAGES_CAPACITY);
                n = 0;
            }
            _ => (),
        }
    }

    // repeat for last entry if not ending with \n
    if n > 0 {
        pages.push(n);
        pages_to_produce.push(pages);
    }

    (map, pages_to_produce)
}
