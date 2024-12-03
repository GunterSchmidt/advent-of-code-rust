/*!

# AoC 2021 Day 01 part 2
See the description of the puzzle at <https://adventofcode.com/2021/day/01>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
Count increased values in a sliding window of 3.

For simplicity this is not performance optimized.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut data = Vec::new();
    let mut n: u32 = 0;
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as u32,
            b'\n' => {
                data.push(n);
                n = 0;
            }
            _ => (),
        }
    }
    if n > 0 {
        data.push(n);
    }

    let mut last = data[0] + data[1] + data[2];
    let mut increase_count = 0;
    for curr in data[1..].windows(3) {
        // dbg!(curr);
        let curr_sum = curr.iter().sum();
        if curr_sum > last {
            increase_count += 1;
        }
        last = curr_sum
    }

    increase_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!("5", solve_puzzle(input));
    }
}
