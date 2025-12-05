# Advent of Code 2025 Day 01

See the description of the puzzle at <https://adventofcode.com/2025/day/1>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2025 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

## The Problem

Turning the dial on a safe left or right with a defined number of clicks. Count zero position.

## Coding Highlights

This code is neither rusty nor concise. It is fast.

The data is parsed as_bytes() allowing to iter ASCII chars fast.
Instead of split a match function is used to save some time.

Since part2 data is sorted, the search can be limited to a small fraction of the second vector.

---
Duration is measured with Criterion without file read but with parsing.

| Test  | Duration |
| ----- | -------- |
| part1 | 0.070 ms |
| part2 | 0.082 ms |
