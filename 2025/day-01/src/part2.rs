/*!

# AoC 2025 Day 01 part 2
See the description of the puzzle at <https://adventofcode.com/2025/day/01>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2025 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Emulate a dial from a safe with numbers 0-99 and count the number of 0 reached.

The data is parsed as_bytes() allowing to iter ASCII chars fast.

*/

use crate::{parse_data, Direction, Num, Rotation};

#[derive(Debug)]
struct Dial {
    pos: Num,
    count_zero_pos: i16,
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) {
        let mut path_zero = rotation.clicks / 100;
        let r = rotation.clicks - path_zero * 100;
        let old_pos = self.pos;
        match rotation.direction {
            Direction::Left => {
                self.pos -= r;
                if self.pos < -99 {
                    self.pos += 100;
                    if self.pos != 0 {
                        path_zero += 1;
                    }
                }
            }
            Direction::Right => {
                self.pos += r;
                if self.pos > 99 {
                    self.pos -= 100;
                    if self.pos != 0 {
                        path_zero += 1;
                    }
                }
            }
        }
        self.count_zero_pos += path_zero;
        if self.pos == 0 {
            self.count_zero_pos += 1;
        } else if old_pos * self.pos < 0 {
            // multiplying to compare sign of old and new. If sign changed, then 0 was passed
            self.count_zero_pos += 1
        };
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
        // println!("{r}: {}, count: {}", dial.pos, dial.count_zero_pos);
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
        assert_eq!("6", solve_puzzle(input));
    }
}
