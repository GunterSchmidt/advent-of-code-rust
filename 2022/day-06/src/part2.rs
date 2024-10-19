/*!
# AoC 2022 Day 6 part 2
See the description of the puzzle at <https://adventofcode.com/2022/day/6>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Surprisingly, the message can start before the start-of-packet marker from part 1 is found.
This eliminates the need to search for the start sequence first.
Since an array is used, the runtime is only a bit slower than part 1, still under 0.003 ms.

Part 1 was rewritten to use the generalized function developed for part 2.

*/

const LEN_START_MESSAGE: usize = 14;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let result = find_end_of_sequence(input, LEN_START_MESSAGE);
    result.to_string()
}

/// Finds the first character sequence without repeated letters in the given length.
/// Returns the first position after the string.
#[inline(always)]
pub fn find_end_of_sequence(input: &str, length: usize) -> usize {
    // Since only small letters are part of the input a small array can do the job,
    // which holds the last position of each letter.
    const A_LOWER_CASE: usize = b'a' as usize;
    let line = input.lines().next().unwrap().as_bytes();
    let mut letter_pos = [0; 26];
    // fill first elements
    // this loop is unnecessary functionality wise but happens to increase performance by 10-15%
    let mut sequence_end = length;
    for (p, &letter) in line[0..length].iter().enumerate() {
        let i = letter as usize - A_LOWER_CASE;
        if letter_pos[i] != 0 {
            sequence_end = letter_pos[i] + length;
        }
        letter_pos[i] = p;
    }
    let mut pos = length;
    for &letter in line[length..].iter() {
        // check if last letters are unique
        let i = letter as usize - A_LOWER_CASE;
        if letter_pos[i] + length > pos {
            let sequence_end_new = letter_pos[i] + length;
            sequence_end = sequence_end_new.max(sequence_end);
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
    fn test_process_0() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!("19", solve_puzzle(input));
    }

    #[test]
    fn test_process_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!("23", solve_puzzle(input));
    }

    #[test]
    fn test_process_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!("23", solve_puzzle(input));
    }

    #[test]
    fn test_process_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!("29", solve_puzzle(input));
    }

    #[test]
    fn test_process_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!("26", solve_puzzle(input));
    }
}
