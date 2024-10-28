/*!

# AoC 2015 Day 5 part 1
See the description of the puzzle at <https://adventofcode.com/2015/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Doing some string parsing.

*/

/// The main function for this puzzle quickly programmed.
pub fn solve_puzzle(input: &str) -> String {
    let nice_sum = input
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut vowels = 0;
            let mut has_double = false;
            let mut last_letter = 0;
            for &c in line {
                match c {
                    b'b' => {
                        if last_letter == b'a' {
                            return 0;
                        }
                    }
                    b'd' => {
                        if last_letter == b'c' {
                            return 0;
                        }
                    }
                    b'q' => {
                        if last_letter == b'p' {
                            return 0;
                        }
                    }
                    b'y' => {
                        if last_letter == b'x' {
                            return 0;
                        }
                    }

                    b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
                    _ => (),
                }
                // check double letter
                if c == last_letter {
                    has_double = true
                }
                last_letter = c;
            }

            if !has_double || vowels < 3 {
                return 0;
            }

            1
        })
        .sum::<u32>();

    nice_sum.to_string()
}

/// The main function for this puzzle quickly programmed.
pub fn solve_puzzle_quickly_programmed(input: &str) -> String {
    let nice_sum = input
        .lines()
        .map(|line| {
            let mut vowels = 0;
            let mut has_double = false;
            let mut last_letter = '-';
            for c in line.chars() {
                // check vowels
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                    _ => (),
                }
                // check double letter
                if c == last_letter {
                    has_double = true
                }
                last_letter = c;
            }

            if has_double && vowels >= 3 {
                // check must not contain
                if line.contains("ab")
                    || line.contains("cd")
                    || line.contains("pq")
                    || line.contains("xy")
                {
                    return 0;
                }
            } else {
                return 0;
            }

            1
        })
        .sum::<u32>();

    nice_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice() {
        let input = "ugknbfddgicrmopn";
        assert_eq!("1", solve_puzzle(input));
    }

    #[test]
    fn test_naughty() {
        let input = "uurcxstgmygtbstg";
        assert_eq!("0", solve_puzzle(input));
    }
}
