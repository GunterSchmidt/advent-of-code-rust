# Advent of Code 2023 Day 10

See the description of the puzzle at <https://adventofcode.com/2023/day/10>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1: Steer through a maze with given directions.

Part 2: Find an area inside the maze.

### Coding Highlights

The inside identification can be done simply by counting the crossed lines. 

This got a bit out of hand. Originally I thougt it would be a good idea to convert
the data into an internal representation with Structs and Enums for good readability. 
While generally a good idea, it costs a lot of performance (compared to 
just using the input data directly) and blew up the source code.

The runtime is still okay, but could be twice as fast.

---

| Test   | Duration |
| -------| -------- |
| part1: | 0.091 ms |
| part2: | 0.155 ms |

