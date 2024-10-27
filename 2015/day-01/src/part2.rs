/*!

# AoC 2015 Day 1 part 2
See the description of the puzzle at <https://adventofcode.com/2015/day/1>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Create a sum based on given characters and stop when -1 is reached.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut floor = 0;
    for (i, c) in input.as_bytes().iter().enumerate() {
        match *c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return (i + 1).to_string();
        }
    }

    panic!("Floor -1 not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "";
        assert_eq!("0", solve_puzzle(input));
    }
}
