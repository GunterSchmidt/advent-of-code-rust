/*!
# AoC 2022 Day 5 part 1
See the description of the puzzle at <https://adventofcode.com/2022/day/5>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Created a separate stack crate so all changes are controlled within that object.
The data is provided top to bottom, but it is easier to handle the stack when the bottom
element is in row 0, therefore the data needs to be reversed.
Since the stack is small, I use an array which is faster than a Vec<Vec<u8>>. I also use u8
for the elements so the stack size is small.

This solution is rather slow as actual objects are created. It would be much faster to just
iter through the items.

*/

use crate::stack::{Stack, StackValue};

#[derive(Clone, Copy)]
pub struct Movement {
    pub from_stack: StackValue,
    pub to_stack: StackValue,
    pub count: StackValue,
}
impl Movement {
    pub fn new(from_stack: StackValue, to_stack: StackValue, count: StackValue) -> Self {
        Self {
            from_stack: from_stack - 1,
            to_stack: to_stack - 1,
            count,
        }
    }
}

pub fn solve_puzzle_v1(input: &str) -> String {
    // Interestingly, version 1 is only 1% slower than version 2 even though all movements are stored.

    // Version 1
    let (mut stack, movements) = parse_input(input);
    for movement in movements.iter() {
        stack.move_crates_single(*movement);
    }

    stack.top_crates()
}

pub fn solve_puzzle_v2(input: &str) -> String {
    let (mut stack, movement_row) = parse_input_stack(input);
    for line in input.lines().skip(movement_row) {
        stack.move_crates_single_by_line(line);
    }

    stack.top_crates()
}

/// Parses the input string and returns the stack and the start row for movement.
pub(crate) fn parse_input_stack(input: &str) -> (Stack, usize) {
    // the input is in reverse order, so store temporarily then add to Stack
    // used height of each stack
    let mut stack_reversed = Stack::default();
    // reading lines from top -> stack is from top to bottom, which is reversed
    for (row, line) in input.lines().enumerate() {
        if !line.is_empty() {
            let line_data = line.as_bytes();
            // do not read column identifier line
            if line_data[1] == b'1' {
                continue;
            };
            let len = line_data.len();
            let mut i = 1;
            let mut col = 0;
            // add values to stack
            while i < len {
                let container = line_data[i];
                if container != b' ' {
                    stack_reversed.stacks[row][col] = container as StackValue;
                }
                col += 1;
                i += 4;
            }
        } else {
            // switch to moves
            // create stack
            return (Stack::new(&stack_reversed, row - 1), row + 1);
        }
    }

    panic!("Read error");
}

pub(crate) fn parse_input(input: &str) -> (Stack, Vec<Movement>) {
    // the input is in reverse order, so store temporarily then add to Stack
    // used height of each stack
    let mut stack = None;
    let mut stack_reversed = Stack::default();
    // rough estimation of capacity for move
    let capacity = input.len() / 19;
    let mut movements = Vec::with_capacity(capacity);
    let mut is_stack = true;
    // reading lines from top -> stack is from top to bottom, which is reversed
    for (row, line) in input.lines().enumerate() {
        if !line.is_empty() {
            if is_stack {
                let line_data = line.as_bytes();
                // do not read column identifier line
                if line_data[1] == b'1' {
                    continue;
                };
                let len = line_data.len();
                let mut i = 1;
                let mut col = 0;
                // let row = self.rows as StackValue;
                while i < len {
                    let container = line_data[i];
                    if container != b' ' {
                        stack_reversed.stacks[row][col] = container as StackValue;
                    }
                    col += 1;
                    i += 4;
                }
            } else {
                let move_info = line.split(' ').collect::<Vec<_>>();
                movements.push(Movement::new(
                    move_info[3].parse().unwrap(),
                    move_info[5].parse().unwrap(),
                    move_info[1].parse().unwrap(),
                ));
            }
        } else if is_stack {
            // switch to moves
            is_stack = false;
            // create stack
            stack = Some(Stack::new(&stack_reversed, row - 1));
        }
    }

    if let Some(stack) = stack {
        (stack, movements)
    } else {
        panic!("read error")
    }
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
        assert_eq!("CMZ", solve_puzzle_v1(input));
    }
}
