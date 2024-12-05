# Advent of Code 2024 Day 03

See the description of the puzzle at <https://adventofcode.com/2024/day/3>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Parse a text.

### Coding Highlights

Here two solutions are presented, the short one using regex and a manual parser.
The manual parser is 75 / 150 times (!) faster than regex.

---

| Test        | Duration  |
| -----       | --------- |
| part1       | 0.0145 ms |
| part1 regex | 1.1115 ms |
| part2       | 0.0125 ms |
| part2 regex | 1.9370 ms |

