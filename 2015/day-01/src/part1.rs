/*!

# AoC 2015 Day 1 part 1
See the description of the puzzle at <https://adventofcode.com/2015/day/1>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt.  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Create a sum based on given characters.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let floor: i32 = input
        .as_bytes()
        .iter()
        .map(|c| match *c {
            b'(' => 1i32,
            b')' => -1i32,
            _ => 0,
        })
        .sum();

    floor.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "(()(()(";
        assert_eq!("3", solve_puzzle(input));
    }
}
