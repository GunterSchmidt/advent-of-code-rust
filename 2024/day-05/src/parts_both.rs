/*!

# AoC 2024 Day 5
See the description of the puzzle at <https://adventofcode.com/2024/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Since part 1 evaluates all good pages and part 2 the bad pages, this is now combined.
Evaluating part 1 with an array does add a non measurable short time.

*/

use crate::parse_data_array;

/// The main function for this puzzle.
pub fn solve_puzzle_array(input: &str) -> (String, String) {
    let (map, orders) = parse_data_array(input);

    let mut result_part1 = 0;
    // 'pages: for pages in orders.iter() {
    //     for page in pages.windows(2) {
    //         if !map[page[0] as usize][page[1] as usize] {
    //             continue 'pages;
    //         }
    //     }
    //     result_part1 += pages[(pages.len() + 1) / 2 - 1];
    // }

    let mut result_part2 = 0;
    'pages: for pages in orders.iter() {
        let mut i = 0;
        for page in pages.windows(2) {
            if !map[page[0] as usize][page[1] as usize] {
                let len = pages.len();
                let mut ordered = [0; 30];
                ordered[0..len].copy_from_slice(&pages[0..len]);
                // let mut ordered = pages.to_vec();
                while i < len - 1 {
                    if !map[ordered[i] as usize][ordered[i + 1] as usize] {
                        if map[ordered[i + 1] as usize][ordered[i] as usize] {
                            // switch elements
                            ordered.swap(i, i + 1);
                            if i > 0 {
                                i = i.wrapping_sub(2);
                            }
                        } else {
                            panic!("Should not happen");
                        }
                    }
                    i = i.wrapping_add(1);
                }
                result_part2 += ordered[(len + 1) / 2 - 1];
                continue 'pages;
            }
            i += 1;
        }
        result_part1 += pages[(pages.len() + 1) / 2 - 1];
    }

    (result_part1.to_string(), result_part2.to_string())
}
