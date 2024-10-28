# Advent of Code 2015 Day 4

See the description of the puzzle at <https://adventofcode.com/2015/day/4>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1/2: Find an md5 Hash with a given start sequence.

### Coding Highlights

This requires md5 hashing logic, here with the [md5](https://docs.rs/md5/latest/md5/) crate.
Since it runs very long also a threaded version was implemented without external crates.

---

| Test           | Duration   |
| -------------- | ---------- |
| part1          |  30.898 ms |
| part1_threaded |  28.833 ms |
| part2          | 124.960 ms |
| part2_threaded |  31.103 ms |

