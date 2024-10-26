# Advent of Code 2023 Day 11

See the description of the puzzle at <https://adventofcode.com/2023/day/11>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1 & 2: Find symbols on a map, shift them and calculate the Manhatten distance.

### Coding Highlights

My solutions are nothing special, but have a reasonable fast runtime.  
Interesting is part2_v1 performance which is 50% slower as part1_v1 only because the numbers are added as 64-bit numbers.  
For part2_v2 this is even worse and makes the faster parsing obsolete.

I included the beautiful solution from Tim Visee as it is concise, fast, uses a nice mathematical algorithm and is very rusty.

What makes Tim's solution so fast?  
* Parsing: Parsing purely from bytes. End of line search replaced by position calculations, as input is a square.  
* Algorithm: Using only the count of the galaxies is genius. It reduces the amount of data to be stored and calculated by an immense factor. The actual calculation is so short it is not measurable, time is used during parsing.  

---

| Test           | Description                    | Duration |
| -------------- | ------------------------------ | -------- |
| part1_v1       | Simple approach                | 0.061 ms |
| part1_v2       | Optimized (but slow algorithm) | 0.052 ms |
| part1_timvisee | Very nice solution, part2 has same speed | 0.011 ms |
| part2_v1       | Basically the same as part1 v1 | 0.086 ms |
| part2_v2       | Basically the same as part1 v2 | 0.120 ms |

