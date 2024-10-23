/*!
# AoC 2023 Day 2 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/2>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Here the line is only parsed once and no data is stored other than the min cubes.
Most importantly, it is all done with the bytes() functionality, which is much
faster than with string and chars.
This is 10x faster than the String version, which is quite interesting.

*/

/// Main function
/// Find minimum number of cubes needed for each game.
/// Multiply number of cubes for each game, then return total sum.
pub fn solve_puzzle(input: &str) -> String {
    let mut power_sum = 0;

    // loop over each game
    for (_game_id, line) in input.lines().enumerate() {
        // parse game data
        // skip game id, not required
        let pos = line.bytes().position(|it| it == b':').unwrap();
        let t_line = line[pos..].as_bytes();

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        // find throw data
        let mut num_cubes: u32 = 0;
        let mut pos_color = 0;
        for (p, char) in t_line.iter().enumerate() {
            match char {
                b'0'..=b'9' => {
                    num_cubes = num_cubes * 10 + (char - b'0') as u32;
                }
                b' ' => {
                    // remember position of color text
                    if num_cubes > 0 {
                        pos_color = p + 1;
                    }
                }
                b',' | b';' => {
                    // first letter is enough
                    match &t_line[pos_color..][0] {
                        b'r' => {
                            if num_cubes > red_min {
                                red_min = num_cubes;
                            }
                        }
                        b'g' => {
                            if num_cubes > green_min {
                                green_min = num_cubes;
                            }
                        }
                        b'b' => {
                            if num_cubes > blue_min {
                                blue_min = num_cubes;
                            }
                        }
                        &_ => (),
                    };
                    // println!("{num_cubes}: {red_min} {green_min} {blue_min}: {}", &t_line[pos_color..]);
                    num_cubes = 0;
                }
                _ => (),
            }
        }
        // add last throw
        match &t_line[pos_color..] {
            // "red" => {
            [b'r', b'e', b'd'] => {
                if num_cubes > red_min {
                    red_min = num_cubes;
                }
            }
            [b'g', b'r', b'e', b'e', b'n'] => {
                if num_cubes > green_min {
                    green_min = num_cubes;
                }
            }
            [b'b', b'l', b'u', b'e'] => {
                if num_cubes > blue_min {
                    blue_min = num_cubes;
                }
            }
            &_ => (),
        }
        // println!("{}: R {}, G {}, B {}", _game_id+1, red_min, green_min, blue_min);
        power_sum += red_min * green_min * blue_min;
    }
    power_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", solve_puzzle(input));
    }
}
