/*!

# AoC 2021 Day 02 part 1
See the description of the puzzle at <https://adventofcode.com/2021/day/02>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
Map and count.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut forward = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (direction, value) = line.split_once(" ").unwrap();
        let v: u32 = value.parse().unwrap();
        match direction.as_bytes()[0] {
            b'f' => forward += v,
            b'd' => depth += v,
            b'u' => depth -= v,
            _ => (),
        }
    }

    let result = forward * depth;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!("150", solve_puzzle(input));
    }
}
