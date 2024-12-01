/*!

# AoC 2024 Day 01 part 2
See the description of the puzzle at <https://adventofcode.com/2024/day/01>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Compare two lists ascending and count matching elements in other list.

This code is neither rusty nor concise. It is fast.

The data is parsed as_bytes() allowing to iter ASCII chars fast.
Instead of split a match function is used to save some time.

Since part2 data is sorted, the search can be limited to a small fraction of the second vector.

*/

type Num = i32;

/// The main function for this puzzle.
pub fn solve_puzzle(input: &str) -> String {
    let mut data_first = Vec::with_capacity(1000);
    let mut data_second = Vec::with_capacity(1000);

    // parse data as_bytes for performance reasons
    let mut n = 0;
    for &c in input.as_bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as Num,
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
    // sorting first allows later faster search
    data_first.sort();
    data_second.sort();

    // This could be done with iter().position, but for performance the search is manually implemented.
    let mut similarity = 0;
    let mut line_similarity = 0;
    let mut search_start = 0;
    let mut last_first = 0;
    for &first in data_first.iter() {
        if last_first == first {
            similarity += first * line_similarity;
        } else {
            line_similarity = 0;
            for (i, &second) in data_second[search_start..].iter().enumerate() {
                if second > first {
                    if line_similarity > 0 {
                        similarity += first * line_similarity;
                        search_start += i;
                    }
                    break;
                } else if second == first {
                    line_similarity += 1;
                }
            }
            last_first = first
        }
    }
    if line_similarity > 0 && data_first.last().unwrap() >= data_second.last().unwrap() {
        similarity += data_first.last().unwrap() * line_similarity;
    }

    similarity.to_string()
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
        assert_eq!("31", solve_puzzle(input));
    }
}
