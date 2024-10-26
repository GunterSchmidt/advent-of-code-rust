/*!
# AoC 2023 Day 11 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/11>.
Many thanks to Eric Wastl for providing these challenges.

GPL-3.0 license
Copyright (c) 2023 Tim Visee
Source code: <https://github.com/timvisee/advent-of-code-2023/blob/master/day11a/src/main.rs>

---
**Coding Highlights**

I included this beautiful solution as it is concise, fast, uses a nice mathematical algorithm and is very rusty.

What makes it so fast?
* Parsing: Parsing purely from bytes. End of line search replaced by position calculations, as input is a square.
* Algorithm: Using only the count of the galaxies is genius. It reduces the amount of data to be stored and
calculated by an immense factor. The actual calculation is not measurable, time is taken during parsing.

Instead of reading all galaxy positions, they are only counted for each row and column as expansion always applies to
all galaxies in a row or column. This allows to also include the expansion in the calculation instead of updating
the positions first.

Changes to code:
* replaced file read
* rename variables
* included comments

*/

const INC: usize = 1;

pub fn solve_puzzle(input: &str) -> String {
    let map = input.as_bytes();

    // parse data
    let size = map.iter().position(|&b| b == b'\n').unwrap();
    // create two vectors which only store the count of each row and column
    let (mut galaxy_count_columns, mut galaxy_count_rows) = (vec![0; size], vec![0; size]);
    map.iter()
        .enumerate()
        .filter(|(_, &b)| b == b'#')
        .for_each(|(pos, _)| {
            galaxy_count_columns[pos % (size + 1)] += 1;
            galaxy_count_rows[pos / (size + 1)] += 1;
        });

    let result = distance(&galaxy_count_columns) + distance(&galaxy_count_rows);
    result.to_string()
}

// Using insight from <https://redd.it/18fqxuq>
#[inline(always)]
fn distance(counts: &[usize]) -> usize {
    let (mut gaps, mut sum, mut items, mut dist) = (0, 0, 0, 0);
    for (i, count) in counts.iter().enumerate() {
        if *count > 0 {
            let expanded = i + INC * gaps;
            dist += count * (items * expanded - sum);
            sum += count * expanded;
            items += count;
        } else {
            gaps += 1;
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", solve_puzzle(input));
    }
}
