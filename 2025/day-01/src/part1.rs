/*!

# AoC 2025 Day 01 part 1
See the description of the puzzle at <https://adventofcode.com/2025/day/1>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Emulate a dial from a safe with numbers 0-99 and count the number of 0 reached.

*/

use crate::{parse_data, Direction, Num, Rotation};

#[derive(Debug)]
struct Dial {
    // The dial pos can be negative, this does not change the 0 count
    // It could be corrected to 100-pos in case it is negative, but the value is never requested.
    pos: Num,
    // putting the counter into the dial surprisingly boosts performance quite a bit
    count_zero_pos: i16,
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation.direction {
            Direction::Left => self.pos -= rotation.clicks,
            Direction::Right => self.pos += rotation.clicks,
        }
        self.pos = self.pos % 100;
        // dbg!(self.pos);
        if self.pos == 0 {
            self.count_zero_pos += 1
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            pos: 50,
            count_zero_pos: 0,
        }
    }
}

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let data = parse_data(input);
    // dbg!(&data);

    let mut dial = Dial::default();
    for r in data {
        dial.rotate(r);
    }

    dial.count_zero_pos.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!("3", solve_puzzle(input));
    }
}
