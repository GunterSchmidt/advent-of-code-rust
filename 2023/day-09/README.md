# Advent of Code 2023 Day 9

See the description of the puzzle at <https://adventofcode.com/2023/day/9>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1: The classical "Find next value of a given row of integers"

Part 2: Find the previous value.

### Algorithmic Highlights

This is done by a simple loop. The amount of data does not yet require a better solution.  
However a faster solution could probably be implemented using 
[Pascal's triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle).  
Since half the time is spent parsing I consider this too academic.

### Coding Highlights

The program is looping over each row separately. This is a small amount of data, so
instead of using a normal vector or array, the [SmallVec](https://docs.rs/smallvec/latest/smallvec/) 
crate is used, which speeds up the logic almost 50%.

---

| Test   | Duration |
| -------| -------- |
| part1: | 0.077 ms |
| part2: | 0.077 ms |

