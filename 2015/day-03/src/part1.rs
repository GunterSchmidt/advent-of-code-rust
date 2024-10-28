/*!

# AoC 2015 Day 3 part 1
See the description of the puzzle at <https://adventofcode.com/2015/day/3>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Move according to given instructions.

*/

use hashbrown::HashSet;
type Position = i32;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut set: HashSet<(Position, Position)> = HashSet::with_capacity(3000);
    let mut pos = (0, 0);
    set.insert((0, 0));
    for &c in input.as_bytes() {
        match c {
            b'<' => pos.0 -= 1 as Position,
            b'>' => pos.0 += 1 as Position,
            b'^' => pos.1 -= 1 as Position,
            b'v' => pos.1 += 1 as Position,
            _ => (),
        }
        set.insert(pos);
    }

    set.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast() {
        let input = "^>v<";
        assert_eq!("4", solve_puzzle(input));
    }
}
