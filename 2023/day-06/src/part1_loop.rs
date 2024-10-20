/*!
# AoC 2023 Day 6 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/6>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This uses a simple loop to calculates the won races depending on button push time.
Basically brute force trying all options.

*/

/// Calculates the won races depending on button push time.
pub fn solve_puzzle(input: &str) -> String {
    // parse data, time is first line, record distance is second line
    let races = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // simple for loop instead of math
    let mut result = 1;
    for i in 1..races[0].len() {
        let mut races_won = 0;
        let time = races[0][i];
        let record = races[1][i];
        for ms in 0..time {
            let distance = (time - ms) * ms;
            if distance > record {
                races_won += 1;
            }
        }
        result *= races_won;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", solve_puzzle(input));
    }
}
