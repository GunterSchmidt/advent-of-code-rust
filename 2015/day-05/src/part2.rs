/*!

# AoC 2015 Day 5 part 2
See the description of the puzzle at <https://adventofcode.com/2015/day/5>.
Many thanks to Eric Wastl for providing these challenges.

MIT License
Copyright (c) 2024 Gunter Schmidt.
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Doing some string parsing.

*/

/// The main function for this.
pub fn solve_puzzle(input: &str) -> String {
    let nice_sum = input
        .as_bytes()
        .split(|c| *c == b'\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut has_duplicate = false;
            let mut has_repeater = false;
            let mut last_letter = line[0];
            let mut prev_last_letter = 0;
            let mut duplicates = [[(0, 0); 10]; 26];
            let mut dup = String::new();
            let mut rep = String::new();
            for (i_line, &c) in line.iter().skip(1).filter(|c| **c >= b'a').enumerate() {
                // check double letter
                if !has_duplicate {
                    let idx = (c - b'a') as usize;
                    let last_letters_for_c = &mut duplicates[idx];
                    for d in last_letters_for_c.iter_mut() {
                        if d.0 == 0 {
                            *d = (last_letter, i_line);
                            break;
                        }
                        if d.0 == last_letter && i_line - d.1 > 1 {
                            //&& (*d != prev_last_letter || c != last_letter)
                            has_duplicate = true;
                            dup.push(last_letter as char);
                            dup.push(c as char);
                            break;
                        }
                    }
                }
                if c == prev_last_letter {
                    has_repeater = true;
                    rep.push(prev_last_letter as char);
                    rep.push(last_letter as char);
                    rep.push(c as char);
                    rep.push(' ')
                }
                prev_last_letter = last_letter;
                last_letter = c;
            }

            // Missed 'cccc'
            // let s = line
            //     .iter()
            //     .filter_map(|c| if *c > 13 { Some(*c as char) } else { None })
            //     .collect::<String>();
            // let mut dup_check = false;
            // for (i, cs) in line.windows(2).enumerate() {
            //     let test = std::str::from_utf8(cs).unwrap();
            //     let start = i + 2;
            //     if start >= s.len() {
            //         break;
            //     }
            //     if s[i + 2..].find(test).is_some() {
            //         dup_check = true;
            //         break;
            //     }
            // }
            // if has_duplicate != dup_check {
            //     println!("*****");
            // }
            // println!(
            //     "{}: {has_duplicate}: {dup}, {has_repeater}: {rep}",
            //     // std::str::from_utf8(line).unwrap(),
            //     &s,
            // );
            if has_duplicate && has_repeater {
                return 1;
            }

            0
        })
        .sum::<u32>();

    nice_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice() {
        let input = "qjhvhtzxzqqjkmpb";
        assert_eq!("1", solve_puzzle(input));
    }

    #[test]
    fn test_naughty() {
        let input = "uurcxstgmygtbstg";
        assert_eq!("0", solve_puzzle(input));
    }
}
