/*!
# AoC 2023 Day 3 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/3>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This is my hand written Kotlin code converted to Rust by Perplexity AI.
Unfortunately all comments got lost.
Part 1 worked immediately.
For part 2 a derive Clone, Copy was missing, and also the adjacent field were not
filled, as in Kotlin I did reuse the result of part 1 and Perplexity did not do that.
Overall quite impressive.

*/

use std::collections::HashMap;
use std::time::Instant;

use aoc_file_reader::read_file;

use crate::FILENAME_PART_1;

#[derive(Debug, Clone, Copy)]
struct NumberData {
    value: i32,
    start_pos: usize,
    end_pos_exclusive: usize,
    is_adjacent: bool,
}

#[derive(Debug)]
struct LineData {
    /// all numbers of that line
    numbers: Vec<NumberData>,
    symbol_map: HashMap<usize, char>,
}

impl LineData {
    fn new(line: &str) -> Self {
        let mut numbers = Vec::new();
        let mut symbol_map = HashMap::new();
        let mut number_started = false;
        let mut start = 0;

        // works, could be faster with as_bytes()
        for (pos, c) in line.char_indices() {
            if c != '.' {
                if c.is_digit(10) {
                    if !number_started {
                        number_started = true;
                        start = pos;
                    }
                } else {
                    if number_started {
                        number_started = false;
                        numbers.push(NumberData {
                            value: line[start..pos].parse().unwrap(),
                            start_pos: start,
                            end_pos_exclusive: pos,
                            is_adjacent: false,
                        });
                    }
                    symbol_map.insert(pos, c);
                }
            } else if number_started {
                number_started = false;
                numbers.push(NumberData {
                    value: line[start..pos].parse().unwrap(),
                    start_pos: start,
                    end_pos_exclusive: pos,
                    is_adjacent: false,
                });
            }
        }

        if number_started {
            numbers.push(NumberData {
                value: line[start..].parse().unwrap(),
                start_pos: start,
                end_pos_exclusive: line.len(),
                is_adjacent: false,
            });
        }

        LineData {
            numbers,
            symbol_map,
        }
    }
}

#[derive(Debug)]
struct GearNumber {
    number_value1: i32,
    number_value2: i32,
    line_no: usize,
    pos_symbol: usize,
    gear_count: i32,
}

fn check_number_is_adjacent_symbol(numbers: &mut [NumberData], pos_symbol: usize) {
    for number in numbers.iter_mut() {
        if number.start_pos <= pos_symbol + 1 && number.end_pos_exclusive >= pos_symbol {
            number.is_adjacent = true;
        }
    }
}

pub fn solve_puzzle_1(input: &str) -> String {
    let mut data: Vec<LineData> = input.lines().map(|s| LineData::new(s)).collect();

    for line_no in 0..data.len() {
        // works, takes even into account borrow issue and clones keys
        let symbol_positions: Vec<usize> = data[line_no].symbol_map.keys().cloned().collect();
        for pos in symbol_positions {
            if line_no > 0 {
                check_number_is_adjacent_symbol(&mut data[line_no - 1].numbers, pos);
            }
            check_number_is_adjacent_symbol(&mut data[line_no].numbers, pos);
            if line_no < data.len() - 1 {
                check_number_is_adjacent_symbol(&mut data[line_no + 1].numbers, pos);
            }
        }
    }

    let result: i32 = data
        .iter()
        .flat_map(|line| &line.numbers)
        .filter(|n| n.is_adjacent)
        .map(|n| {
            // println!("r = {}", n.value);
            n.value
        })
        .sum();

    result.to_string()
}

fn check_number_is_potential_gear(
    numbers: &[NumberData],
    pos_symbol: usize,
    line_no: usize,
    gears: &mut Vec<GearNumber>,
) {
    for number in numbers {
        if number.start_pos <= pos_symbol + 1 && number.end_pos_exclusive >= pos_symbol {
            if let Some(gear) = gears
                .iter_mut()
                .find(|g| g.line_no == line_no && g.pos_symbol == pos_symbol)
            {
                gear.gear_count += 1;
                if gear.gear_count == 2 {
                    gear.number_value2 = number.value;
                }
            } else {
                gears.push(GearNumber {
                    number_value1: number.value,
                    number_value2: 0,
                    line_no,
                    pos_symbol,
                    gear_count: 1,
                });
            }
        }
    }
}

pub fn solve_puzzle_2(input: &str) -> String {
    let mut data: Vec<LineData> = input.lines().map(|s| LineData::new(s)).collect();

    // added the adjacent logic
    // Perplexity missed that because in Kotlin I was using the dataset produced in part 1.
    for line_no in 0..data.len() {
        // works, takes even into account borrow issue and clones keys
        let symbol_positions: Vec<usize> = data[line_no].symbol_map.keys().cloned().collect();
        for pos in symbol_positions {
            if line_no > 0 {
                check_number_is_adjacent_symbol(&mut data[line_no - 1].numbers, pos);
            }
            check_number_is_adjacent_symbol(&mut data[line_no].numbers, pos);
            if line_no < data.len() - 1 {
                check_number_is_adjacent_symbol(&mut data[line_no + 1].numbers, pos);
            }
        }
    }

    let mut gears = Vec::new();

    for (line_no, line) in data.iter().enumerate() {
        for (&pos, &symbol) in &line.symbol_map {
            if symbol == '*' {
                if line_no > 0 {
                    check_number_is_potential_gear(
                        &data[line_no - 1]
                            .numbers
                            .iter()
                            .filter(|n| n.is_adjacent)
                            .cloned()
                            .collect::<Vec<_>>(),
                        pos,
                        line_no,
                        &mut gears,
                    );
                }
                check_number_is_potential_gear(
                    &data[line_no]
                        .numbers
                        .iter()
                        .filter(|n| n.is_adjacent)
                        .cloned()
                        .collect::<Vec<_>>(),
                    pos,
                    line_no,
                    &mut gears,
                );
                if line_no < data.len() - 1 {
                    check_number_is_potential_gear(
                        &data[line_no + 1]
                            .numbers
                            .iter()
                            .filter(|n| n.is_adjacent)
                            .cloned()
                            .collect::<Vec<_>>(),
                        pos,
                        line_no,
                        &mut gears,
                    );
                }
            }
        }
    }

    let result: i32 = gears
        .iter()
        .filter(|g| g.gear_count == 2)
        .map(|g| g.number_value1 * g.number_value2)
        .sum();

    result.to_string()
}

#[allow(dead_code)]
fn main() {
    let input = read_file(FILENAME_PART_1);

    println!("Results for 2023, day 3:");

    let start = Instant::now();
    let result1 = solve_puzzle_1(&input);
    let duration1 = start.elapsed();
    println!("Result of part 1: {}, duration = {:?}", result1, duration1);

    let start = Instant::now();
    let result2 = solve_puzzle_2(&input);
    let duration2 = start.elapsed();
    println!("Result of part 2: {}, duration = {:?}", result2, duration2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", solve_puzzle_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", solve_puzzle_2(input));
    }
}
