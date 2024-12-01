/*!

# AoC 2024 Day 01 part 1
See the description of the puzzle at <https://adventofcode.com/2024/day/01>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Compare two lists ascending and calculate delta.

This code is neither rusty nor concise. It is fast.

The data is parsed as_bytes() allowing to iter ASCII chars fast.
Instead of split a match function is used to save some time.

*/

type Num = i32;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut data_first = Vec::with_capacity(1000);
    let mut data_second = Vec::with_capacity(1000);

    // parse data as_bytes for performance reasons
    // since only the delta is relevant, the ASCII code is kept
    let mut n = 0;
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + c as Num,
            b' ' => {
                if n > 0 {
                    data_first.push(n);
                    n = 0;
                }
            }
            b'\n' => {
                data_second.push(n);
                n = 0;
            }
            _ => (),
        }
    }
    if n > 0 {
        data_second.push(n);
    }
    assert_eq!(data_first.len(), data_second.len());
    data_first.sort();
    data_second.sort();

    let mut delta = 0;
    for i in 0..data_first.len() {
        let d = (data_second[i] - data_first[i]).abs();
        delta += d;
    }

    delta.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!("11", solve_puzzle(input));
    }
}
