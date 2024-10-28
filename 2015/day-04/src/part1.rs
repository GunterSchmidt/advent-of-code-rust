/*!

# AoC 2015 Day 4 part 1
See the description of the puzzle at <https://adventofcode.com/2015/day/4>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Find an md5 Hash with a given start sequence.

Since it runs very long also a threaded version was implemented without external crates.

*/

use std::{
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::WITH_OUTPUT_PRINT;

/// End loop here if no value was found.
pub const MAX: usize = 2_000_000;
/// CPUs reported by system or this number, whatever is lower.
const MAX_THREADS: usize = 20;
/// Range for loop within thread, will check stop each round.
const RANGE_SIZE: usize = 100_000;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let min_md5 = scan_range(input, 0, MAX as u32, StopZeros::Five).unwrap();
    if WITH_OUTPUT_PRINT {
        println!("solve_puzzle: {min_md5}");
    }

    min_md5.number.to_string()
}

/// The main function for this puzzle as threaded version.
pub fn solve_puzzle_threaded(input: &str) -> String {
    let num_cpus: usize = thread::available_parallelism().unwrap().into();
    let num_cpus = num_cpus.min(MAX_THREADS);

    let min_md5 = solve_puzzle_threads(input, StopZeros::Five, num_cpus as u8);
    if WITH_OUTPUT_PRINT {
        println!("solve_puzzle_threaded: {min_md5}");
    }

    min_md5.number.to_string()
}

/// Splits the challenge in parts and runs each part in a separate thread
pub fn solve_puzzle_threads(input: &str, stop_num_zeros: StopZeros, num_cpus: u8) -> MinMd5 {
    let secret_key = input.as_bytes();
    let mut check_key: [u8; 20] = [0; 20];
    let s_len = secret_key.len();
    check_key[0..s_len].copy_from_slice(secret_key);

    let num_cpus = num_cpus as usize;
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(num_cpus);

    let size = MAX / num_cpus;
    let (tx, rx) = mpsc::channel();
    let best_find = Arc::new(Mutex::new(u32::MAX));

    // Spawn threads
    for t in 0..num_cpus {
        let start = t * size;
        let end = if t == num_cpus - 1 { MAX } else { start + size };
        let key = input.to_string();
        if WITH_OUTPUT_PRINT {
            println!("Starting thread {t} from {start} to {end}");
        }
        let tx = tx.clone();
        let best_find_local = best_find.clone();
        let zeros = stop_num_zeros;
        let handle = thread::spawn(move || {
            let packages = (end - start) / RANGE_SIZE;
            for i in 0..packages {
                let s = start + i * RANGE_SIZE;
                let e = if i == packages - 1 {
                    end
                } else {
                    s + RANGE_SIZE
                };
                // println!("scan: thread {t} from {start} to {end} : {s} to {e}");
                let min_md5 = scan_range(&key, s as u32, e as u32, zeros);
                tx.send(min_md5).unwrap();
                if *best_find_local.lock().unwrap() < e as u32 {
                    // already a better number known
                    if WITH_OUTPUT_PRINT {
                        println!("Stopping thread {t} from {start} to {end} at {e}");
                    }
                    break;
                }
            }
            // println!("Finished thread from {start} to {end}");
        });
        handles.push(handle);
    }
    // drop because tx itself is not used
    drop(tx);

    let mut min_md5 = MinMd5::new(input);
    for received in rx.iter().flatten() {
        // threading does not work for this as the first result is required
        if WITH_OUTPUT_PRINT {
            println!("*** Found: {}", &received);
        }
        if stop_num_zeros == StopZeros::Five {
            if received.min_n > 0 && (received.number < min_md5.number || min_md5.number == 0) {
                min_md5 = received;
                if WITH_OUTPUT_PRINT {
                    println!("*** Took: {}", &min_md5);
                }
            }
        } else if received.min_n == 0 && (received.number < min_md5.number || min_md5.number == 0) {
            min_md5 = received;
            // ending threads after first entry is received which starts with 000000
            // if STOP_AFTER_FIRST {
            //     *best_find.lock().unwrap() = true;
            //     println!("*** Ended: {} ***", &min_md5);
            // }
            *best_find.lock().unwrap() = min_md5.number;
            if WITH_OUTPUT_PRINT {
                println!("*** Took: {}", &min_md5);
            }
        }
    }

    min_md5
}

/// Scans the range and returns the first found value matching the number of zeros.
pub fn scan_range(
    secret_key: &str,
    start: u32,
    end: u32,
    stop_num_zeros: StopZeros,
) -> Option<MinMd5> {
    let b_secret_key = secret_key.as_bytes();
    let mut check_key: [u8; 20] = [0; 20];
    let s_len = b_secret_key.len();
    check_key[0..s_len].copy_from_slice(b_secret_key);

    let mut min_md5 = MinMd5::new(secret_key);
    for i in start..end {
        let mut buffer = itoa::Buffer::new();
        let digits = buffer.format(i).as_bytes();
        let len = digits.len();
        check_key[s_len..s_len + len].copy_from_slice(digits);
        let md5 = md5::compute(&check_key[0..s_len + len]);
        if md5[0] == 0 && md5[1] == 0 && md5[2] < 16 {
            let n: u32 = md5[2] as u32;
            if stop_num_zeros == StopZeros::Five {
                min_md5.min_n = n;
                min_md5.number = i;
                return Some(min_md5);
            }
            if n < min_md5.min_n {
                min_md5.min_n = n;
                min_md5.number = i;
                if n == 0 {
                    return Some(min_md5);
                }
                // println!("{i}: min: {:x}", min_md5);
            }
        }
    }

    None
}

#[derive(PartialEq, Clone, Copy)]
pub enum StopZeros {
    Five,
    Six,
}

pub struct MinMd5 {
    pub secret_key: String,
    pub number: u32,
    pub min_n: u32,
}

impl MinMd5 {
    fn new(secret_key: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            number: 0,
            min_n: u32::MAX,
        }
    }
}

impl Display for MinMd5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.secret_key.to_owned() + &self.number.to_string();
        let md5 = md5::compute(key.as_bytes());
        write!(f, "{} / {key}: md5 = {:x}", self.number, md5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "abcdef";
        assert_eq!("609043", self::solve_puzzle(input));
    }

    #[test]
    fn test_single() {
        let input = "abcdef609043";
        let md5 = md5::compute(input.as_bytes());
        let md5_string = format!("{:x}", md5);
        assert_eq!("000001dbbfa3a5c83a2d506429c7b00e", md5_string);
    }
}
