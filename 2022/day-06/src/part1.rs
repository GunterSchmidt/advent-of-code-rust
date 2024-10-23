/*!
# AoC 2022 Day 6 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/6>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

The trick is to use a small array to store the last position. This is extremely fast
and the result is found in less than 0.002 ms!

*/

use crate::part2::find_end_of_sequence;
const LEN_START_SEQUENCE: usize = 4;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    // let result = find_start_of_message(input);
    let result = find_end_of_sequence(input, LEN_START_SEQUENCE);
    result.to_string()
}

#[allow(dead_code)]
fn find_start_of_message(input: &str) -> usize {
    // One could use a HashMap, but that is overdone and slow for small sizes.
    // Since only small letters are part of the input a small array can do the job,
    // which holds the last position of each letter.
    const A_LOWER_CASE: usize = b'a' as usize;
    let line = input.lines().next().unwrap().as_bytes();
    let mut letter_pos = [0; 26];
    // fill first elements
    let mut sequence_end = LEN_START_SEQUENCE;
    for (p, &letter) in line[0..4].iter().enumerate() {
        // let i = (letter - b'a') as usize; // calculation in u8 is slower than in usize
        let i = letter as usize - A_LOWER_CASE;
        if letter_pos[i] != 0 {
            sequence_end = letter_pos[i] + LEN_START_SEQUENCE;
        }
        letter_pos[i] = p;
    }
    let mut pos = LEN_START_SEQUENCE;
    for &letter in line[LEN_START_SEQUENCE..].iter() {
        // check if last letters are unique
        // let i = (letter - b'a') as usize; // calculation in u8 is slower than in usize
        let i = letter as usize - A_LOWER_CASE;
        if letter_pos[i] + LEN_START_SEQUENCE > pos {
            if letter_pos[i] + LEN_START_SEQUENCE > sequence_end {
                sequence_end = letter_pos[i] + LEN_START_SEQUENCE;
            }
        } else if pos >= sequence_end {
            return sequence_end + 1;
        }
        letter_pos[i] = pos;
        pos += 1;
    }
    panic!("no start sequence found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!("7", solve_puzzle(input));
    }

    #[test]
    fn test_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!("5", solve_puzzle(input));
    }

    #[test]
    fn test_2() {
        let input = "npdvjthqldpwncqszvftbrmjlhg";
        assert_eq!("5", solve_puzzle(input));
    }

    #[test]
    fn test_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!("10", solve_puzzle(input));
    }

    #[test]
    fn test_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!("11", solve_puzzle(input));
    }
}
