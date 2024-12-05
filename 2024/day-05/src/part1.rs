/*!

# AoC 2024 Day 5 part 1
See the description of the puzzle at <https://adventofcode.com/2024/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

*/

use crate::{parse_data_array, parse_data_hashmap};

/// The main function for this puzzle.
pub fn solve_part1_array(input: &str) -> String {
    let (map, orders) = parse_data_array(input);

    let mut result = 0;
    'pages: for pages in orders.iter() {
        for page in pages.windows(2) {
            if !map[page[0] as usize][page[1] as usize] {
                continue 'pages;
            }
        }
        result += pages[(pages.len() + 1) / 2 - 1];
    }

    result.to_string()
}

/// The main function for this puzzle.
pub fn solve_part1_hashmap(input: &str) -> String {
    let (map, orders) = parse_data_hashmap(input);
    // let max: usize = map.values().map(|v| v.len()).max().unwrap();
    // dbg!(max);

    let mut result = 0;
    'pages: for pages in orders.iter() {
        for page in pages.windows(2) {
            if let Some(set) = map.get(&page[0]) {
                if !set.contains(&page[1]) {
                    continue 'pages;
                }
            } else {
                continue 'pages;
            }
        }
        result += pages[(pages.len() + 1) / 2 - 1];
    }

    result.to_string()
}

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
        assert_eq!("143", solve_part1_array(input));
        assert_eq!("143", solve_part1_hashmap(input));
    }
}
