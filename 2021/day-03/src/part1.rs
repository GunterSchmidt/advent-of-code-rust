/*!

# AoC 2021 Day 03 part 1
See the description of the puzzle at <https://adventofcode.com/2021/day/03>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Count bits and find most and least common one.

*/

type DataType = u32;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    // get number of bits by analyzing first line
    let bits = input
        .as_bytes()
        .iter()
        .position(|&c| c != b'0' && c != b'1')
        .unwrap();

    // work in array
    let mut data = [0; 1000];
    // read data and convert into number
    let mut len = 0;
    for (i, line) in input.lines().enumerate() {
        data[i] = DataType::from_str_radix(line, 2).unwrap();
        len = i;
    }
    len += 1;

    // find most common bit for each bit position
    // use bitshift
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut mask = 1;
    let half = len / 2;
    for _ in 0..bits {
        let mut count = 0;
        for &n in data[0..len].iter() {
            // count 1s
            if n & mask != 0 {
                count += 1;
            }
        }
        if count >= half {
            gamma_rate |= mask;
        } else {
            epsilon_rate |= mask;
        }
        mask = mask << 1;
    }
    // dbg!(gamma_rate, epsilon_rate);

    let result = gamma_rate * epsilon_rate;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
        assert_eq!("198", solve_puzzle(input));
    }
}
