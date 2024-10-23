/*!
# AoC 2023 Day 2 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/2>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Nothing special here.

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
        // game_id is same a line number, saves parsing step
        // parse game data
        let mut is_valid_game = true;
        let game_data: Vec<&str> = line.split(&[':', ';']).collect();
        // loop over each throw
        'game: for games_str in &game_data[1..] {
            // parse data
            let cube_data: Vec<&str> = games_str.split(',').collect();
            // loop over each cube color
            for cube in cube_data {
                let c_str: Vec<&str> = cube.trim().split(' ').collect();
                let count: i32 = c_str[0].parse().expect("number expected");
                match c_str[1] {
                    "red" => {
                        if count > red_max {
                            is_valid_game = false;
                            break 'game;
                        }
                    }
                    "green" => {
                        if count > green_max {
                            is_valid_game = false;
                            break 'game;
                        }
                    }
                    "blue" => {
                        if count > blue_max {
                            is_valid_game = false;
                            break 'game;
                        }
                    }
                    &_ => (),
                }
            }
        }
        // println!("Games {no} is valid = {is_valid}");
        if is_valid_game {
            count_valid_games += game_id as u32 + 1;
        }
    }
    //println!("Games {:?}", games.len());

    count_valid_games.to_string()
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
        assert_eq!("8", solve_puzzle(input));
    }
}
