# Advent of Code 2024 Day11

See the description of the puzzle at <https://adventofcode.com/2024/day/11>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Create a list of elements by specific rules and count number of created elements.

Actually creating each element will run too long.

### Coding Highlights

To solve this puzzle fast, a HashMap is used for all the created elements and their count.
Whenever an existing number is created or changed into, only the count needs to be changed.
Also in the next round this element is handled only once. In the end this is 20 times faster
for part 1 than a naive approach and the only way to solve part 2.

---

| Test        | Duration |
| ----------- | -------- |
| part1       | 0.080 ms |
| part1 naive | 1.930 ms |
| part2       | 2.630 ms |
| together    | 2.630 ms |

