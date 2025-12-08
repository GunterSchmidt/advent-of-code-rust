use std::fmt::Display;

pub mod part1;
pub mod part2;

// input file names
// path is set in aoc_file_reader::read_file.rs, but defaulted to "..\res"
pub const FILENAME_PART_1: &str = "input_day_03.txt";
pub const FILENAME_PART_2: &str = FILENAME_PART_1;

#[derive(Debug)]
pub struct BatteryBank {
    pub bank: Vec<u8>,
}

impl BatteryBank {
    fn joltage_two(&self) -> u32 {
        let mut first = self.bank[0];
        let mut second = 0;
        for &b in &self.bank[1..self.bank.len() - 1] {
            if b > first {
                first = b;
                second = 0;
                continue;
            }
            if b > second {
                second = b;
            }
        }
        let b = *self.bank.last().unwrap();
        if b > second {
            second = b;
        }

        ((first - b'0') * 10 + second - b'0') as u32
    }

    fn joltage_twelve(&self) -> u64 {
        let mut best = [0u8; 12];
        let l = self.bank.len();
        'bat: for (i, &b) in self.bank.iter().enumerate() {
            let mut p = if l - i > 12 { 0 } else { 12 - (l - i) };
            loop {
                if b > best[p] {
                    best[p] = b;
                    best[p + 1..].fill(0);
                    continue 'bat;
                }
                p += 1;
                if p == 12 {
                    break;
                }
            }
        }

        // merge number
        let mut joltage = 0;
        for b in best {
            joltage = joltage * 10 + (b - b'0') as u64;
        }

        joltage
    }
}

impl Display for BatteryBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in &self.bank {
            write!(f, "{}", b as char)?
        }
        // write!(f, ", j: {}", self.joltage_two())
        Ok(())
    }
}

/// Parses the input str to the required data format.
pub fn parse_data(input: &str) -> Vec<BatteryBank> {
    let mut data = Vec::new();

    let iter = input.split_ascii_whitespace();
    for bank in iter {
        data.push(BatteryBank {
            bank: bank.as_bytes().to_vec(),
        });
    }

    data
}
