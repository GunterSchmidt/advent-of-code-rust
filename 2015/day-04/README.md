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

PC Windows native:

| Test            | Duration   |
| --------------  | ---------- |
| part1           |  30.898 ms |
| part1_threaded* |  28.833 ms |
| part2           | 124.960 ms |
| part2_threaded* |  31.103 ms |
 
 
Notebook Windows native:
 
| Test            | Duration   |
| --------------  | ---------- |
| part1           |  34.936 ms |
| part1_threaded* |  52.381 ms |
| part2           | 143.780 ms |
| part2_threaded* |  56.639 ms |
 
Notebook Linux Mint native:
 
| Test            | Duration   |
| --------------  | ---------- |
| part1           |  34.828 ms |
| part1_threaded* |  56.992 ms |
| part2           | 143.410 ms |
| part2_threaded* |  48.991 ms |
 
Notebook Linux Mint VirtualBox under Windows 10:
 
| Test            | Duration   |
| --------------  | ---------- |
| part1           |  37.510 ms |
| part1_threaded* |  85.784 ms |
| part2           | 148.390 ms |
| part2_threaded* |  89.260 ms |
 
Notebook Linux Manjaro VirtualBox under Windows 10:
 
| Test            | Duration   |
| --------------  | ---------- |
| part1           |  38.937 ms |
| part1_threaded* |  75.557 ms |
| part2           | 145.870 ms |
| part2_threaded* |  87.113 ms |

* PC Windows/Linux native: 6 CPU + 6 Hyper-Threading
* Notebook Windows/Linux native: 4 CPU + 4 Hyper-Threading
* Linux VirtualBox: 4 CPU