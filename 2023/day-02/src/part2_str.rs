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

/// Find minimum number of cubes needed for each game.
/// Multiply number of cubes for each game, then return total sum.
pub fn solve_puzzle(input: &str) -> String {
    let mut result_sum = 0;

    // loop over each game
    for line in input.lines() {
        // game_id is same a line number, saves parsing step
        // parse game data
        let game_data: Vec<&str> = line.split(&[':', ';']).collect();
        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;
        // loop over each throw
        for games_str in &game_data[1..] {
            let cube_data: Vec<&str> = games_str.split(',').collect();
            // loop over each cube color
            for cube in cube_data {
                let c_str: Vec<&str> = cube.trim().split(' ').collect();
                let num_cubes: i32 = c_str[0].parse().expect("number expected");
                match c_str[1] {
                    "red" => {
                        if num_cubes > red_min {
                            red_min = num_cubes;
                        }
                    }
                    "green" => {
                        if num_cubes > green_min {
                            green_min = num_cubes;
                        }
                    }
                    "blue" => {
                        if num_cubes > blue_min {
                            blue_min = num_cubes;
                        }
                    }
                    &_ => (),
                }
                // games.push(build_game(red, green, blue));
            }
        }
        // println!("Games {no} is valid = {is_valid}");
        result_sum += red_min * green_min * blue_min;
    }
    //println!("Games {:?}", games.len());

    result_sum.to_string()
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
        assert_eq!("2286", solve_puzzle(input));
    }
}
