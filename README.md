# Advent of Code 2023 Day 9

See the description of the puzzles at <https://adventofcode.com/>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

# Advent of Code in Rust
Advent of Code Solutions written in Rust with emphasis on performance aspects.

## Puzzle data files
As of the Advent of Code rules, the puzzle inputs are not part of this repository.

Under each year a /res folder needs to be presents and the filename should be 
in the format 'input_day_01.txt'. 
Otherwise please change the const values in the lib.rs files.

# Lessons Learned

## Parsing
A lot of puzzles require very small amounts of time to solve. More than half of the time is spent
to parse the data. Using input.as_bytes() greatly improves speed as all puzzle inputs are in ASCII.

Avoiding to import the data into vectors or the like and instead working with the input data directly is also very beneficial.

## Small vectors
Small vectors can be handled on the stack with crate [SmallVec](https://docs.rs/smallvec/latest/smallvec/).
The amount of time saved can be 30-50%. Arrays are an alternative, too.

## Numbers

Interesting enough, 32-Bit calculations can be faster than 64-bit calculations, even on 64-bit systems.
