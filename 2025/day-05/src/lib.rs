use std::ops::RangeInclusive;

pub mod part1;
pub mod part2;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_05.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

pub type Range = RangeInclusive<u64>;

/// Elf Database
#[derive(Debug, Default)]
pub struct Ingredients {
    fresh: Vec<Range>,
    // available is only used for part 1, could have been an option
    available: Vec<u64>,
    // merged is only used for part 2
    merged: Option<Vec<Range>>,
}

impl Ingredients {
    fn sort_fresh(&mut self) {
        self.fresh.sort_by(|a, b| a.start().cmp(b.start()));
    }

    /// Merged ranges. Sorting ranges avoids a lot of search and merging of ranges.
    fn merge_ranges(&mut self) -> &Option<Vec<RangeInclusive<u64>>> {
        if !self.fresh.is_empty() {
            self.sort_fresh();

            let mut mr: Vec<Range> = vec![self.fresh.first().unwrap().clone()];
            for r in self.fresh.iter().skip(1) {
                // dbg!(mr.last().unwrap(), r);
                let end = *mr.last().unwrap().end();
                if *r.start() > end + 1 {
                    // gap between ranges
                    mr.push(r.clone());
                } else if *r.end() > end {
                    // must be inside, just make range longer
                    let x = mr.last_mut().unwrap();
                    *x = RangeInclusive::new(*x.start(), *r.end());
                }
            }

            self.merged = Some(mr);
        }
        &self.merged
    }

    /// Merged ranges, but count only values. This avoids the need for a Vec.
    /// This has no measurable time (< 1 microsecond).
    fn merged_ranges_count(&mut self) -> u64 {
        if self.fresh.is_empty() {
            return 0;
        };

        self.sort_fresh();
        let mut total = 0;

        let mut mr = self.fresh.first().unwrap().clone();
        for r in self.fresh.iter().skip(1) {
            // dbg!(mr.last().unwrap(), r);
            if *r.start() > *mr.end() + 1 {
                // gap between ranges
                total += mr.end() - mr.start() + 1;
                mr = r.clone();
            } else if r.end() > mr.end() {
                // must be inside, just make range longer
                mr = RangeInclusive::new(*mr.start(), *r.end());
            }
        }

        total + mr.end() - mr.start() + 1
    }
}

/// Parses the input str to the required data format.
/// Requires floor to be a square in size with max 140 chars.
pub fn parse_data(input: &str, parse_available: bool) -> Ingredients {
    let mut ing = Ingredients::default();

    // parse data as_bytes for performance reasons
    let mut is_first = true;
    let mut from = 0;
    let mut to = 0;
    let mut start_ing = 0;
    for (i, &c) in input.as_bytes().iter().enumerate() {
        match c {
            b'0'..=b'9' => {
                if is_first {
                    from = from * 10 + (c - b'0') as u64;
                } else {
                    to = to * 10 + (c - b'0') as u64;
                }
            }
            b'-' => is_first = false,
            b'\n' => {
                if from > 0 {
                    ing.fresh.push(RangeInclusive::new(from, to));
                    from = 0;
                    to = 0;
                } else {
                    // blank line has no data
                    start_ing = i + 1;
                    break;
                }
                is_first = true;
            }
            _ => (),
        }
    }

    if !parse_available {
        return ing;
    }

    // ingredients
    let mut id = 0;
    for &c in &input.as_bytes()[start_ing..] {
        match c {
            b'0'..=b'9' => {
                id = id * 10 + (c - b'0') as u64;
            }
            b'\n' => {
                if id > 0 {
                    ing.available.push(id);
                    id = 0;
                } else {
                    // blank line has no data
                    break;
                }
            }
            _ => (),
        }
    }
    if id > 0 {
        ing.available.push(id);
    }

    ing
}
