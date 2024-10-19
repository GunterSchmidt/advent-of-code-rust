use crate::part1::Movement;

pub const STACK_ROWS: usize = 50;
pub const STACK_COLUMNS: usize = 9;
pub type StackValue = u8;
type StackRow = [StackValue; STACK_COLUMNS];
/// Full Stack of items in array, Rows is a guess, but size seems small.
type StackArray = [StackRow; STACK_ROWS];

/// Stack for the crates, bottom crates need to be in row 0.
pub struct Stack {
    pub stacks: StackArray,
    /// number of crates in that column
    pub crates_in_col: [usize; STACK_COLUMNS],
}

impl Stack {
    // creates a new stack where the order of the elements is reversed
    pub fn new(stack_reversed: &Stack, rows: usize) -> Self {
        let mut stack = Self::default();
        for (row, row_data) in stack_reversed.stacks[0..rows].iter().rev().enumerate() {
            stack.stacks[row] = *row_data;
        }
        // fill top_crate
        for col in 0..STACK_COLUMNS {
            for row in (0..rows).rev() {
                if stack.stacks[row][col] != 0 {
                    stack.crates_in_col[col] = row + 1;
                    break;
                }
            }
        }
        stack
    }

    /// return the top crates of the current configuration
    pub fn top_crates(&self) -> String {
        // get last filled col; wondering if this is really a no cost abstraction
        let cols = STACK_COLUMNS
            - self
                .crates_in_col
                .iter()
                .rev()
                .position(|&it| it != 0)
                .unwrap();
        let mut s = String::new();
        for (col, &row) in self.crates_in_col[0..cols].iter().enumerate() {
            let c = if row > 0 {
                self.stacks[row - 1][col] as char
            } else {
                ' '
            };
            s.push(c);
        }
        s
    }

    /// moves crates accordingly; does not validate
    /// force copy of Movement, as it is smaller than the reference
    pub fn move_crates_single(&mut self, movement: Movement) {
        let from_stack = movement.from_stack as usize;
        let to_stack = movement.to_stack as usize;
        for _ in 0..movement.count {
            self.crates_in_col[from_stack] -= 1;
            self.stacks[self.crates_in_col[to_stack]][to_stack] =
                self.stacks[self.crates_in_col[from_stack]][from_stack];
            self.crates_in_col[to_stack] += 1;
        }
    }

    /// moves crates accordingly; does not validate
    /// force copy of Movement, as it is smaller than the reference
    pub fn move_crates_single_by_line(&mut self, line: &str) {
        let move_info = line.split(' ').collect::<Vec<_>>();
        let from_stack: usize = move_info[3].parse::<usize>().unwrap() - 1;
        let to_stack: usize = move_info[5].parse::<usize>().unwrap() - 1;
        let count: usize = move_info[1].parse().unwrap();
        for _ in 0..count {
            self.crates_in_col[from_stack] -= 1;
            self.stacks[self.crates_in_col[to_stack]][to_stack] =
                self.stacks[self.crates_in_col[from_stack]][from_stack];
            self.crates_in_col[to_stack] += 1;
        }
    }

    /// moves crates accordingly; does not validate
    /// force copy of Movement, as it is smaller than the reference
    pub fn move_crates_multiple(&mut self, movement: Movement) {
        let from_stack = movement.from_stack as usize;
        let to_stack = movement.to_stack as usize;
        self.crates_in_col[from_stack] -= movement.count as usize;
        let mut from_col = self.crates_in_col[from_stack];
        for _ in 0..movement.count {
            self.stacks[self.crates_in_col[to_stack]][to_stack] = self.stacks[from_col][from_stack];
            self.crates_in_col[to_stack] += 1;
            from_col += 1;
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            stacks: [[0; STACK_COLUMNS]; STACK_ROWS],
            crates_in_col: [0; STACK_COLUMNS],
        }
    }
}
