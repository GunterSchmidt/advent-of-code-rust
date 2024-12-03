/*!

# AoC 2021 Day 03 part 2
See the description of the puzzle at <https://adventofcode.com/2021/day/03>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Count bits and find most and least common one with filter.

*/

type DataType = u32;
const ARRAY_SIZE: usize = 1000;

pub fn solve_puzzle(input: &str) -> String {
    // get number of bits by analyzing first line
    let bits = input
        .as_bytes()
        .iter()
        .position(|&c| c != b'0' && c != b'1')
        .unwrap();

    // work in array
    let mut data = [0; ARRAY_SIZE];
    // read data and convert into number
    let mut len = 0;
    for (i, line) in input.lines().enumerate() {
        data[i] = DataType::from_str_radix(line, 2).unwrap();
        len = i;
    }
    len += 1;

    // find most common bit for each bit position
    let gamma_rate = filtered_value(&data, len, bits, true);
    let epsilon_rate = filtered_value(&data, len, bits, false);
    // dbg!(gamma_rate, epsilon_rate);

    let result = gamma_rate * epsilon_rate;
    result.to_string()
}

fn filtered_value(
    data: &[DataType],
    len: usize,
    bits: usize,
    filter_most_common: bool,
) -> DataType {
    // copy array, so values can be removed
    // let mut nums: [DataType; ARRAY_SIZE] = [0; ARRAY_SIZE];
    // nums[0..len].copy_from_slice(&data[0..len]);
    // retain only works for vec, manually implementing is not worth it
    let mut nums = data[0..len].to_vec();

    let mut mask = (2 as DataType).pow(bits as u32 - 1);
    for _ in 0..bits {
        let mut count = 0;
        for &n in nums.iter() {
            // count 1s
            if n & mask != 0 {
                count += 1;
            }
        }
        if filter_most_common {
            if count >= (nums.len() + 1) / 2 {
                nums.retain(|&it| it & mask != 0)
            } else {
                nums.retain(|&it| it & mask == 0)
            }
        } else {
            if count < (nums.len() + 1) / 2 {
                nums.retain(|&it| it & mask != 0)
            } else {
                nums.retain(|&it| it & mask == 0)
            }
        }
        if nums.len() == 1 {
            return nums[0];
        }
        mask = mask >> 1;
    }
    panic!("Should not happen")
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
        assert_eq!("230", solve_puzzle(input));
    }
}
