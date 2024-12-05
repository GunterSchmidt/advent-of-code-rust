/*!

# AoC 2024 Day 5 part 2
See the description of the puzzle at <https://adventofcode.com/2024/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

*/

use crate::{parse_data_array, parse_data_hashmap};

/// The main function for this puzzle.
pub fn solve_part2_array(input: &str) -> String {
    let (map, orders) = parse_data_array(input);

    let mut result = 0;
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
                result += ordered[(len + 1) / 2 - 1];
                continue 'pages;
            }
            i += 1;
        }
    }

    result.to_string()
}

/// The main function for this puzzle.
pub fn solve_part2_hashmap(input: &str) -> String {
    let (map, orders) = parse_data_hashmap(input);

    let mut result = 0;
    for pages in orders.iter() {
        let mut ok = true;
        let mut i = 0;
        for page in pages.windows(2) {
            if let Some(set) = map.get(&page[0]) {
                if !set.contains(&page[1]) {
                    ok = false;
                    break;
                }
            } else {
                ok = false;
                break;
            }
            i += 1;
        }
        if !ok {
            //     result += pages[(pages.len() + 1) / 2 - 1];
            // } else {
            // order_pages(&pages);
            // order pages
            // ordering in array is slightly faster and does not alter original data
            let len = pages.len();
            let mut ordered = [0; 30];
            ordered[0..len].copy_from_slice(&pages[0..len]);
            // let mut ordered = pages.to_vec();
            while i < len - 1 {
                if let Some(set) = map.get(&ordered[i]) {
                    if !set.contains(&ordered[i + 1]) {
                        if let Some(set) = map.get(&ordered[i + 1]) {
                            if set.contains(&ordered[i]) {
                                // switch elements
                                ordered.swap(i, i + 1);
                                if i > 0 {
                                    i = i.wrapping_sub(2);
                                }
                            } else {
                                panic!("Should not happen")
                            }
                        } else {
                            panic!("Should not happen")
                        }
                    }
                } else {
                    if let Some(set) = map.get(&ordered[i + 1]) {
                        if set.contains(&ordered[i]) {
                            // switch elements
                            ordered.swap(i, i + 1);
                            if i > 0 {
                                i = i.wrapping_sub(2);
                            }
                        } else {
                            panic!("Should not happen")
                        }
                    } else {
                        panic!("Should not happen")
                    }
                }
                i = i.wrapping_add(1);
            }
            result += ordered[(len + 1) / 2 - 1];
        }
    }

    result.to_string()
}

// fn order_pages(pages: &[DataType]) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!("123", solve_part2_array(input));
        assert_eq!("123", solve_part2_hashmap(input));
    }
}
