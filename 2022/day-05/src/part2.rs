/*!
# AoC 2022 Day 5 part 2
See the description of the puzzle at <https://adventofcode.com/2022/day/5>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Just a small difference in the crate movements (move_crates_multiple).

*/

use crate::part1::parse_input;

// replace this with part 1 coding and adapt it
#[allow(unused_variables)]

pub fn process(input: &str) -> String {
    let (mut stack, movements) = parse_input(input);
    for movement in movements.iter() {
        stack.move_crates_multiple(*movement);
    }

    stack.top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        assert_eq!("MCD", process(input));
    }
}
