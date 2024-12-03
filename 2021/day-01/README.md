# Advent of Code 2021 Day 01

See the description of the puzzle at <https://adventofcode.com/2021/day/01>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1: Count increased values.

Part 1 fast uses an individual parser, which is much faster since ASCII parsing can be done.

Part 2: Count increased values in a sliding window of 3.


---

| Test       | Duration |
| -----      | -------- |
| part1      | 0.036 ms |
| part1 fast | 0.011 ms |
| part2      | 0.014 ms |

