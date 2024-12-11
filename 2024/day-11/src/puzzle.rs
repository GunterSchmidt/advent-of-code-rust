/*!

# AoC 2024 Day 11 part 1
See the description of the puzzle at <https://adventofcode.com/2024/day/11>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

To solve this puzzle fast, a HashMap is used for all the created elements and their count.
Whenever an existing number is created or changed into, only the count needs to be changed.
Also in the next round this element is handled only once. In the end this is 20 times faster
for part 1 than a naive approach and the only way to solve part 2.

*/

use crate::{num_digits, parse_data, DataInt};
use hashbrown::HashMap;

/// Solve part 1 including parsing.
pub fn solve_part1_standalone(input: &str) -> String {
    let data = parse_data(input);

    // solve_part1(data, 25)
    let result = solve(data, 25, 0);
    result.0.to_string()
}

/// The puzzle requires the number of stones only, there is not need for order.
/// For performance reasons all numbers which are created multiple times are handled together
/// in a HashMap. This is much faster than the 'create all' approach.
/// Returns (Full, Part Count)
pub fn solve(data: Vec<DataInt>, blinks_full: usize, blinks_part: usize) -> (DataInt, DataInt) {
    // HashMap holding the number (key) and its count.
    let mut number_map: HashMap<DataInt, DataInt> = HashMap::with_capacity(10000);
    let mut count_part = 0;
    for number in data {
        number_map.insert(number, 1);
    }
    for blink in 0..blinks_full {
        if blink == blinks_part {
            count_part = number_map.values().sum::<DataInt>();
        }
        // copy the HashMap data into a vector, so the count does not get messed up during update
        let numbers = Vec::from_iter(number_map.iter().map(|(&k, &v)| (k, v)));
        // println!(
        //     "{:?}, count {} -> {:?}",
        //     numbers.len(),
        //     number_map.values().sum::<u32>(),
        //     numbers
        // );
        // clear the HashMap as the old numbers will disappear anyhow
        number_map.clear();

        for (d, c) in numbers.iter() {
            let new_value = if *d == 0 {
                1
            } else {
                let digits = num_digits(*d as u64);
                // let digits = d.ilog10() + 1;
                if digits & 1 == 0 {
                    // split in two
                    // let div = (10 as DataInt).pow(digits as u32 / 2);
                    let div = match digits {
                        2 => 10,
                        4 => 100,
                        6 => 1000,
                        8 => 10000,
                        10 => 100000,
                        12 => 1000000,
                        _ => (10 as DataInt).pow(digits / 2),
                    };
                    let right = d % div;
                    if let Some(new) = number_map.get_mut(&right) {
                        *new += c;
                    } else {
                        number_map.insert(right, *c);
                    }

                    (d - right) / div
                } else {
                    d * 2024
                }
            };

            if let Some(new) = number_map.get_mut(&new_value) {
                *new += c;
            } else {
                number_map.insert(new_value, *c);
            }
        }
    }
    let count_full = number_map.values().sum::<DataInt>();

    (count_full, count_part)
}

/// The puzzle requires the number of stones, there is not need for order.
/// This is the naive and slow approach creating all elements.
pub fn solve_part1_create_all(data: Vec<DataInt>, blinks: usize) -> String {
    let mut numbers = Vec::with_capacity(10000);
    let mut count = 0;
    for number in data {
        numbers.clear();
        numbers.push(number);

        for _ in 0..blinks {
            for i in 0..numbers.len() {
                let d = numbers[i];
                if d == 0 {
                    numbers[i] = 1;
                } else {
                    let digits = num_digits(d as u64);
                    // let digits = d.ilog10() + 1;
                    if digits & 1 == 0 {
                        // split in two
                        // let div = (10 as DataInt).pow(digits as u32 / 2);
                        let div = match digits {
                            2 => 10,
                            4 => 100,
                            6 => 1000,
                            8 => 10000,
                            10 => 100000,
                            12 => 1000000,
                            _ => (10 as DataInt).pow(digits / 2),
                        };
                        let last = d % div;
                        numbers.push(last);
                        numbers[i] = (d - last) / div;
                    } else {
                        numbers[i] = d * 2024;
                    }
                }
            }
            // println!("{:?}", numbers);
        }
        count += numbers.len();
        // println!("{:?}", numbers.len());
    }

    count.to_string()
}

/// Solve part 1 including parsing.
pub fn solve_part2_standalone(input: &str) -> String {
    let data = parse_data(input);
    let result = solve(data, 75, 0);
    result.0.to_string()
}

/// The main function for this puzzle.
pub fn solve_both(input: &str) -> (String, String) {
    let data = parse_data(input);
    let result = solve(data, 75, 25);
    (result.1.to_string(), result.0.to_string())
}

// /// The main function for this puzzle.
// pub fn solve_part2(data: Vec<u64>) -> String {
//     0.to_string()
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!("55312", solve_part1_standalone(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("0", solve_part2_standalone(INPUT));
    }
}
