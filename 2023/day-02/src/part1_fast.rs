/*!
# AoC 2023 Day 2 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/2>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Here the line is only parsed once and no data is stored.
Most importantly, it is all done with the bytes() functionality, which is much
faster than with string and chars.
This is 10x faster than the String version, but is more complex to program.

*/

/// Main function
/// Find number of games which are possible with given number of cubes.
/// Returns sum of ids of the games.
pub fn solve_puzzle(input: &str) -> String {
    // max cubes available
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let mut count_valid_games = 0;
    // loop over each game
    for (game_id, line) in input.lines().enumerate() {
        // parse game data
        let mut is_valid_game = true;
        // game_id is same a line number, saves parsing step
        let pos = line.bytes().position(|it| it == b':').unwrap() + 1;
        let t_line = line[pos..].as_bytes();
        'game: for throw in t_line.split(|&it| it == b';') {
            for cubes in throw.split(|&it| it == b',') {
                // 1st digit
                let mut n = cubes[1] - b'0';
                // has 2nd digit?
                let c = if cubes[2] == b' ' {
                    3
                } else {
                    n = n * 10 + cubes[2] - b'0';
                    4
                };
                // first letter of color
                match cubes[c] {
                    b'r' => {
                        if n > red_max {
                            is_valid_game = false;
                            break 'game;
                        }
                    }
                    b'g' => {
                        if n > green_max {
                            is_valid_game = false;
                            break 'game;
                        }
                    }
                    b'b' => {
                        if n > blue_max {
                            is_valid_game = false;
                            break 'game;
                        }
                    }
                    _ => panic!("read error"),
                }
            }
        }

        if is_valid_game {
            count_valid_games += game_id as u32 + 1;
        }
    }

    count_valid_games.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", solve_puzzle(input));
    }
}
