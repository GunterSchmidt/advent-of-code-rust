/*!

# AoC 2015 Day 3 part 2
See the description of the puzzle at <https://adventofcode.com/2015/day/3>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Move according to given instructions with two players.

*/

use hashbrown::HashSet;
type Position = i16;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut set: HashSet<(Position, Position)> = HashSet::with_capacity(3000);
    let mut pos_santa = (0, 0);
    let mut pos_robo = (0, 0);
    set.insert((0, 0));
    for (i, &c) in input.as_bytes().iter().enumerate() {
        if i % 2 == 0 {
            match c {
                b'<' => pos_santa.0 -= 1 as Position,
                b'>' => pos_santa.0 += 1 as Position,
                b'^' => pos_santa.1 -= 1 as Position,
                b'v' => pos_santa.1 += 1 as Position,
                _ => (),
            }
            set.insert(pos_santa);
        } else {
            match c {
                b'<' => pos_robo.0 -= 1 as Position,
                b'>' => pos_robo.0 += 1 as Position,
                b'^' => pos_robo.1 -= 1 as Position,
                b'v' => pos_robo.1 += 1 as Position,
                _ => (),
            }
            set.insert(pos_robo);
        }
    }

    set.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast() {
        let input = "^>v<";
        assert_eq!("3", solve_puzzle(input));
    }
}
