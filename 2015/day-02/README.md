# Advent of Code 2015 Day 2

See the description of the puzzle at <https://adventofcode.com/2015/day/2>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1/2: Calculate an area with given values.

### Coding Highlights

For part1 three solutions are implemented, the normal 'rusty' one and a faster one 
using at atoi and a more low level implementation.  
Working with UTF-8 is very time consuming and takes more than 7x longer than the hand
implemented solution.

Programming the fast one is much more time consuming as it is error prone and requires
much more testing. Therefore it should be done only in very performance critical 
and stable (rare code/data changes) environments.

---

| Test        | Duration |
| ----------- | -------- |
| part1_rusty | 0.118 ms |
| part1_atoi  | 0.084 ms |
| part1_fast  | 0.014 ms |
| part2       | 0.018 ms |

/*!
