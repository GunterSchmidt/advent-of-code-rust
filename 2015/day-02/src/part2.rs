/*!

# AoC 2015 Day 2 part 2
See the description of the puzzle at <https://adventofcode.com/2015/day/2>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Calculate an area with given values.

*/

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut l_w_h = [0; 3];
    let sq_feet: u32 = input
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

                let ribbon_feet;
                // find min2 of three, could sort, but this is faster
                if l_w_h[0] < l_w_h[1] {
                    if l_w_h[1] < l_w_h[2] {
                        ribbon_feet = l_w_h[0] + l_w_h[1];
                    } else {
                        ribbon_feet = l_w_h[0] + l_w_h[2];
                    }
                } else {
                    if l_w_h[0] < l_w_h[2] {
                        ribbon_feet = l_w_h[0] + l_w_h[1];
                    } else {
                        ribbon_feet = l_w_h[1] + l_w_h[2];
                    }
                }
                ribbon_feet * 2 + l_w_h.iter().product::<u32>()
            } else {
                0
            }
        })
        .sum();

    sq_feet.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast() {
        let input = "2x3x4";
        assert_eq!("34", solve_puzzle(input));
    }
}
