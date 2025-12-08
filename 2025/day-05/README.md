# Advent of Code 2025 Day 5

See the description of the puzzle at <https://adventofcode.com/2025/day/5>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2025 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

## The Problem

Working with Ranges

## Coding Highlights

This code is focussing on performance, one could have written it more idiomatic.

---
Duration is measured with Criterion without file read (input String) but with parsing.
Parsing is included in part1 and part2 duration but separated here to show the impact.

| Test   | Duration |
| -----  | -------- |
| parse1 | 0.020 ms |
| part1  | 0.076 ms |
| parse2 | 0.006 ms |
| part2  | 0.008 ms |
