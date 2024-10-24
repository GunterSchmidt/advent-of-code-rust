/*!
# AoC 2023 Day 10 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/10>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Like part1 way to structured.

*/

use hashbrown::HashSet;

use crate::part1::*;

// Part 1
// Solve a map path, find enclosed fields.
// Uses array to solve_puzzle faster than vectors as file fits in memory
pub fn solve_puzzle(input: &str) -> String {
    // input to array
    let mut maze = Maze::parse_to_maze(input);

    // find inside a polygon by counting crossing lines
    // assumes all pipes are used, which is not the case, therefore find only used pipes
    // this doubles the used space on the stack
    let mut maze_used: MazeArray = [[Maze::GROUND_POINT; MAX_COLUMNS]; MAX_ROWS];

    let pos_start_field = maze.start_field.pos;
    let mut _steps = 1;
    let mut current = maze.second_field.clone();
    let mut last_field: Field = current.clone();
    maze_used[current.pos.0][current.pos.1] = current.pipe;
    while current.pos != pos_start_field {
        last_field = current;
        current = maze.next_pipe_v2(&last_field);
        maze_used[current.pos.0][current.pos.1] = current.pipe;
        _steps += 1;
    }
    maze.maze = maze_used;

    // check from left to right each line, count inside and outside depending on char
    // one pipe switches inside to outside: |I|O||O
    //
    // upper corners have no effect
    // | and lower corners L and J switch inside to outside

    // for this to work S needs to be set correctly; only interested if L or J
    // both will come back as North
    if maze.start_field.to_dir == Direction::North {
        if last_field.to_dir == Direction::West {
            maze.maze[pos_start_field.0][pos_start_field.1] = Maze::NORTH_EAST_L;
        } else if last_field.to_dir == Direction::East {
            maze.maze[pos_start_field.0][pos_start_field.1] = Maze::NORTH_WEST_J;
        }
    }

    let mut inside_positions: HashSet<Position> = HashSet::new();
    for row in 0..maze.rows {
        let mut is_inside = false;
        for col in 0..maze.columns {
            let c = maze.maze[row][col];
            if is_inside && c == Maze::GROUND_POINT {
                inside_positions.insert((row, col));
            } else if matches!(
                c,
                Maze::NORTH_SOUTH_PIPE | Maze::NORTH_EAST_L | Maze::NORTH_WEST_J
            ) {
                is_inside = !is_inside;
            }
            // dbg!(row, col, c, is_inside, inside_positions.len());
        }
    }

    inside_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "...........
.F-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.S--J.L--J.
...........";
        assert_eq!("4", solve_puzzle(input));
    }

    #[test]
    fn test_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", solve_puzzle(input));
    }

    #[test]
    fn test_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!("10", solve_puzzle(input));
    }
}
