# Advent of Code 2024 Day 02

See the description of the puzzle at <https://adventofcode.com/2024/day/02>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1: Filter list for ascending/descending steps 1 to 3.

Part 2: Filter list for ascending/descending steps 1 to 3 with one error allowed.

### Coding Highlights

There are suprising many constellations which is difficult to check. As such
once an error occurs, the two constellations with one item removed are checked.

The code uses an array which is much faster than a vector. When creating a second
array memcopy is used (copy_within).

---

| Test  | Duration |
| ----- | -------- |
| part1 | 0.020 ms |
| part2 | 0.028 ms |

