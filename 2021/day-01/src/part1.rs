/*!

# AoC 2021 Day 01 part 1
See the description of the puzzle at <https://adventofcode.com/2021/day/01>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
Count increased values.

Solve puzzle fast uses as_bytes for performance gains.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut iter = input.lines();
    let mut last = iter.next().unwrap().parse().unwrap();
    let mut increase_count = 0;
    for line in iter {
        let curr: i32 = line.parse().unwrap();
        // dbg!(curr);
        if curr > last {
            increase_count += 1;
        }
        last = curr
    }

    increase_count.to_string()
}

/// The main function for this puzzle.
pub fn solve_puzzle_fast(input: &str) -> String {
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

    let mut last = data[0];
    let mut increase_count = 0;
    for &curr in data[1..].iter() {
        // dbg!(curr);
        if curr > last {
            increase_count += 1;
        }
        last = curr
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
        assert_eq!("7", solve_puzzle(input));
    }
}
