/*!
# AoC 2023 Day 10 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/10>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This got a bit out of hand. Originally I thougt it would be a good idea to convert
the data into an internal representation with Structs and Enums for good readability. 
While generally a good idea, it costs a lot of performance (compared to 
just using the input data directly) and blew up the source code.

The runtime is still okay, but could be twice as fast.

*/

// define array size
pub const MAX_COLUMNS: usize = 150;
pub const MAX_ROWS: usize = 150;

pub type Position = (usize, usize);
// array could be u8 with converted cars (enum) which would reduce stack space,
// but this will unlikely result in any speed up
pub type MazeArray = [[u8; MAX_COLUMNS]; MAX_ROWS];

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    pub fn to_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::None => Direction::None,
        }
    }
    pub fn to_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub pos: Position,
    pub to_dir: Direction,
    pub pipe: u8,
}

pub struct Maze {
    pub maze: MazeArray,
    pub rows: usize,
    pub columns: usize,
    pub start_field: Field,
    pub second_field: Field,
}

impl Maze {
    pub const START_S: u8 = b'S';
    pub const NORTH_SOUTH_PIPE: u8 = b'|';
    pub const WEST_EAST_DASH: u8 = b'-';
    pub const NORTH_EAST_L: u8 = b'L';
    pub const NORTH_WEST_J: u8 = b'J';
    pub const SOUTH_WEST_7: u8 = b'7';
    pub const SOUTH_EAST_F: u8 = b'F';
    pub const GROUND_POINT: u8 = b'.';
    pub const INSIDE_LEFT: i16 = 1;
    pub const INSIDE_RIGHT: i16 = -1;

    /// Parses the given input into a maze array.
    pub fn parse_to_maze(input: &str) -> Self {
        // create empty maze
        let mut maze = [[Maze::GROUND_POINT; MAX_COLUMNS]; MAX_ROWS];
        let mut pos_start_point = (0, 0);
        let mut rows = 0;
        for (line_no, line) in input.lines().enumerate() {
            for (col, &c) in line.as_bytes().iter().enumerate() {
                maze[line_no][col] = c;
                if c == Maze::START_S {
                    pos_start_point = (line_no, col);
                }
            }
            rows = line_no;
        }
        rows += 1;
        let columns = input.lines().next().unwrap().len();

        let mut maze = Self {
            maze,
            rows,
            columns,
            start_field: Field {
                pos: pos_start_point,
                to_dir: Direction::None,
                pipe: Maze::START_S,
            },
            second_field: Field {
                pos: (0, 0),
                to_dir: Direction::None,
                pipe: Maze::START_S,
            },
        };

        // find a second field after S to start the tour
        maze.second_field = maze.get_second_point();

        maze
    }

    /// Finds first element after 'S', the start field.
    /// S is connected at exactly two sides,
    /// first side found is used
    fn get_second_point(&mut self) -> Field {
        let row = self.start_field.pos.0;
        let col = self.start_field.pos.1;

        // check north
        if row > 0 {
            match self.maze[row - 1][col] {
                Maze::NORTH_SOUTH_PIPE => {
                    self.start_field.to_dir = Direction::North;
                    return Field {
                        pos: (row - 1, col),
                        to_dir: Direction::North,
                        pipe: Maze::NORTH_SOUTH_PIPE,
                    };
                }
                Maze::SOUTH_WEST_7 => {
                    self.start_field.to_dir = Direction::North;
                    return Field {
                        pos: (row - 1, col),
                        to_dir: Direction::West,
                        pipe: Maze::SOUTH_WEST_7,
                    };
                }
                Maze::SOUTH_EAST_F => {
                    self.start_field.to_dir = Direction::North;
                    return Field {
                        pos: (row - 1, col),
                        to_dir: Direction::East,
                        pipe: Maze::SOUTH_EAST_F,
                    };
                }
                _ => (),
            }
        }

        // check east
        if col < self.columns - 1 {
            match self.maze[row][col + 1] {
                Maze::WEST_EAST_DASH => {
                    return Field {
                        pos: (row, col + 1),
                        to_dir: Direction::East,
                        pipe: Maze::WEST_EAST_DASH,
                    }
                }
                Maze::NORTH_WEST_J => {
                    return Field {
                        pos: (row, col + 1),
                        to_dir: Direction::North,
                        pipe: Maze::NORTH_WEST_J,
                    }
                }
                Maze::SOUTH_WEST_7 => {
                    return Field {
                        pos: (row, col + 1),
                        to_dir: Direction::South,
                        pipe: Maze::SOUTH_WEST_7,
                    }
                }
                _ => (),
            }
        }

        // check south
        if row < self.rows - 1 {
            match self.maze[row + 1][col] {
                Maze::NORTH_SOUTH_PIPE => {
                    return Field {
                        pos: (row + 1, col),
                        to_dir: Direction::South,
                        pipe: Maze::NORTH_SOUTH_PIPE,
                    }
                }
                Maze::NORTH_WEST_J => {
                    return Field {
                        pos: (row + 1, col),
                        to_dir: Direction::West,
                        pipe: Maze::NORTH_WEST_J,
                    }
                }
                Maze::NORTH_EAST_L => {
                    return Field {
                        pos: (row + 1, col),
                        to_dir: Direction::East,
                        pipe: Maze::NORTH_EAST_L,
                    }
                }
                _ => (),
            }
        }

        // check west
        if col > 0 {
            match self.maze[row][col - 1] {
                Maze::WEST_EAST_DASH => {
                    return Field {
                        pos: (row, col - 1),
                        to_dir: Direction::West,
                        pipe: Maze::WEST_EAST_DASH,
                    }
                }
                Maze::NORTH_EAST_L => {
                    return Field {
                        pos: (row, col - 1),
                        to_dir: Direction::North,
                        pipe: Maze::NORTH_EAST_L,
                    }
                }
                Maze::SOUTH_EAST_F => {
                    return Field {
                        pos: (row, col - 1),
                        to_dir: Direction::South,
                        pipe: Maze::SOUTH_EAST_F,
                    }
                }
                _ => (),
            }
        }
        panic!("no pipe found")
    }

    #[inline(always)]
    pub fn next_pipe(&self, field: Field) -> Field {
        let pos = match field.to_dir {
            Direction::North => (field.pos.0 - 1, field.pos.1),
            Direction::East => (field.pos.0, field.pos.1 + 1),
            Direction::South => (field.pos.0 + 1, field.pos.1),
            Direction::West => (field.pos.0, field.pos.1 - 1),
            Direction::None => field.pos,
        };
        let c = self.maze[pos.0][pos.1];
        match c {
            Maze::SOUTH_EAST_F => {
                if field.to_dir == Direction::North {
                    return Field {
                        pos,
                        to_dir: Direction::East,
                        pipe: Maze::SOUTH_EAST_F,
                    };
                } else {
                    return Field {
                        pos,
                        to_dir: Direction::South,
                        pipe: Maze::SOUTH_EAST_F,
                    };
                }
            }
            Maze::NORTH_WEST_J => {
                if field.to_dir == Direction::East {
                    return Field {
                        pos,
                        to_dir: Direction::North,
                        pipe: Maze::NORTH_WEST_J,
                    };
                } else {
                    return Field {
                        pos,
                        to_dir: Direction::West,
                        pipe: Maze::NORTH_WEST_J,
                    };
                }
            }
            Maze::SOUTH_WEST_7 => {
                if field.to_dir == Direction::East {
                    return Field {
                        pos,
                        to_dir: Direction::South,
                        pipe: Maze::SOUTH_WEST_7,
                    };
                } else {
                    return Field {
                        pos,
                        to_dir: Direction::West,
                        pipe: Maze::SOUTH_WEST_7,
                    };
                }
            }
            Maze::NORTH_SOUTH_PIPE => {
                if field.to_dir == Direction::South {
                    return Field {
                        pos,
                        to_dir: Direction::South,
                        pipe: Maze::NORTH_SOUTH_PIPE,
                    };
                } else {
                    return Field {
                        pos,
                        to_dir: Direction::North,
                        pipe: Maze::NORTH_SOUTH_PIPE,
                    };
                }
            }
            Maze::NORTH_EAST_L => {
                if field.to_dir == Direction::South {
                    return Field {
                        pos,
                        to_dir: Direction::East,
                        pipe: Maze::NORTH_EAST_L,
                    };
                } else {
                    return Field {
                        pos,
                        to_dir: Direction::North,
                        pipe: Maze::NORTH_EAST_L,
                    };
                }
            }
            Maze::WEST_EAST_DASH => {
                if field.to_dir == Direction::West {
                    return Field {
                        pos,
                        to_dir: Direction::West,
                        pipe: Maze::WEST_EAST_DASH,
                    };
                } else {
                    return Field {
                        pos,
                        to_dir: Direction::East,
                        pipe: Maze::WEST_EAST_DASH,
                    };
                }
            }
            Maze::START_S => {
                return Field {
                    pos,
                    to_dir: Direction::North,
                    pipe: Maze::START_S,
                }
            }
            _ => todo!("pipe {c} not implemented"),
        }
    }

    #[inline(always)]
    pub fn next_pipe_v2(&self, field: &Field) -> Field {
        let pos = match field.to_dir {
            Direction::North => (field.pos.0 - 1, field.pos.1),
            Direction::East => (field.pos.0, field.pos.1 + 1),
            Direction::South => (field.pos.0 + 1, field.pos.1),
            Direction::West => (field.pos.0, field.pos.1 - 1),
            Direction::None => field.pos,
        };
        let c = self.maze[pos.0][pos.1];
        let mut field_new = Field {
            pos,
            to_dir: Direction::None,
            pipe: c,
        };
        match c {
            Maze::SOUTH_EAST_F => {
                field_new.to_dir = if field.to_dir == Direction::North {
                    Direction::East
                } else {
                    Direction::South
                }
            }
            Maze::NORTH_WEST_J => {
                field_new.to_dir = if field.to_dir == Direction::East {
                    Direction::North
                } else {
                    Direction::West
                }
            }
            Maze::SOUTH_WEST_7 => {
                field_new.to_dir = if field.to_dir == Direction::East {
                    Direction::South
                } else {
                    Direction::West
                }
            }
            Maze::NORTH_SOUTH_PIPE => {
                field_new.to_dir = if field.to_dir == Direction::South {
                    Direction::South
                } else {
                    Direction::North
                }
            }
            Maze::NORTH_EAST_L => {
                field_new.to_dir = if field.to_dir == Direction::South {
                    Direction::East
                } else {
                    Direction::North
                }
            }
            Maze::WEST_EAST_DASH => {
                field_new.to_dir = if field.to_dir == Direction::West {
                    Direction::West
                } else {
                    Direction::East
                }
            }

            Maze::START_S => field_new.to_dir = Direction::North,

            _ => todo!("pipe {c} not implemented"),
        }
        field_new
    }

    // fn is_corner(c: u8) -> bool {
    //     matches!(
    //         c,
    //         Maze::SOUTH_EAST_F | Maze::NORTH_WEST_J | Maze::SOUTH_WEST_7 | Maze::NORTH_EAST_L
    //     )
    // }
}

// Part 1
// Solve a map path
// Uses arrays to solve_puzzle faster than vectors as file fits in memory
pub fn solve_puzzle(input: &str) -> String {
    // input to array
    let maze = Maze::parse_to_maze(input);
    // dbg!(maze.pos_start_point);

    // follow path and count steps
    let mut current = maze.second_field.clone();
    let mut steps = 1;
    let pos_start_field = maze.start_field.pos;
    while current.pos != pos_start_field {
        current = maze.next_pipe_v2(&current);
        steps += 1;
        // dbg!(&current);
    }
    // dbg!(steps);

    let steps_to_furthest = steps / 2;

    steps_to_furthest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", solve_puzzle(input));
    }
}
