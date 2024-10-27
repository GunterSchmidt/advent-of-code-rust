/*!

# AoC 2015 Day 2 part 1
See the description of the puzzle at <https://adventofcode.com/2015/day/2>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Calculate an area with given values.

Here three solutions are implemented, the normal 'rusty' one and a faster one
using at atoi and a more low level implementation.
Working with UTF-8 is very time consuming and takes >7x longer than the hand
implemented solution.

*/

/// The main function for this puzzle.
pub fn solve_puzzle_rusty(input: &str) -> String {
    let sq_feet: u32 = input
        .lines()
        .map(|line| {
            let l_w_h = line
                .split("x")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let mut min = l_w_h[0] * l_w_h[1];
            let mut sq_feet = min * 2;
            let wh = l_w_h[1] * l_w_h[2];
            sq_feet += wh * 2;
            min = min.min(wh);
            let hl = l_w_h[2] * l_w_h[0];
            sq_feet += hl * 2;
            min = min.min(hl);
            sq_feet += min;

            sq_feet
        })
        .sum();

    sq_feet.to_string()
}

/// The main function for this puzzle.
pub fn solve_puzzle_rusty_atoi(input: &str) -> String {
    let sq_feet: u32 = input
        .lines()
        .map(|line| {
            let l_w_h = line
                .as_bytes()
                .split(|c| *c == b'x')
                .map(|n| atoi::atoi::<u32>(n).unwrap())
                .collect::<Vec<_>>();
            let mut min = l_w_h[0] * l_w_h[1];
            let mut sq_feet = min * 2;
            let wh = l_w_h[1] * l_w_h[2];
            sq_feet += wh * 2;
            min = min.min(wh);
            let hl = l_w_h[2] * l_w_h[0];
            sq_feet += hl * 2;
            min = min.min(hl);
            sq_feet += min;

            sq_feet
        })
        .sum();

    sq_feet.to_string()
}

/// The main function for this puzzle.
pub fn solve_puzzle_fast(input: &str) -> String {
    let mut l_w_h = [0; 3];
    let sq_feet = input
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut n = 0;
            let mut i = 0;
            for &c in line.iter() {
                match c {
                    b'0'..=b'9' => n = 10 * n + c as u32 - b'0' as u32,
                    _ => {
                        if i < 2 {
                            l_w_h[i] = n;
                            n = 0;
                            i += 1;
                        }
                    }
                }
            }
            if line.len() > 4 {
                l_w_h[2] = n;
                let mut min = l_w_h[0] * l_w_h[1];
                let mut sq_feet = min * 2;
                let wh = l_w_h[1] * l_w_h[2];
                sq_feet += wh * 2;
                min = min.min(wh);
                let hl = l_w_h[2] * l_w_h[0];
                sq_feet += hl * 2;
                min = min.min(hl);
                sq_feet + min
            } else {
                0
            }
        })
        .sum::<u32>();

    sq_feet.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast() {
        let input = "2x3x4";
        assert_eq!("58", solve_puzzle_fast(input));
    }

    #[test]
    fn test_rusty() {
        let input = "2x3x4";
        assert_eq!("58", solve_puzzle_rusty(input));
    }
}
