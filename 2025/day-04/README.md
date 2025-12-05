# Advent of Code 2025 Day 4

See the description of the puzzle at <https://adventofcode.com/2025/day/4>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2025 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

## The Problem

Identifying fields on a 2D map based on the surrounding field's contents.

## Coding Highlights

This code is focussing on performance and uses an array for fast stack operation.
One could have written it more rusty.

The data is parsed as_bytes() allowing to iter ASCII chars fast.
Instead of split a match function is used to save some time.

Since part2 data is sorted, the search can be limited to a small fraction of the second vector.

---
Duration is measured with Criterion without file read (input String) but with parsing.
Parsing is included in part1 and part2 duration but separated here to show the impact.

| Test  | Duration |
| ----- | -------- |
| parse | 0.073 ms |
| part1 | 0.170 ms |
| part2 | 1.940 ms |
