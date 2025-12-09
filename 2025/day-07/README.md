# Advent of Code 2025 Day 7

See the description of the puzzle at <https://adventofcode.com/2025/day/7>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2025 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

## The Problem

Follow rays through a downward array with splits.

## Coding Highlights

This code is focussing on performance, one could have written it more idiomatic.

The data is parsed as_bytes() allowing to iter ASCII chars fast.
Here only every second line is required for the logic, which saves a lot of parsing time.

Instead of following each ray, all rays per line are accumulated in an array, which are only a few calculations.

---
Duration is measured with Criterion without file read (input String) but with parsing.
Parsing is included in part1 and part2 duration but separated here to show the impact.

Extremely short runtimes with only 7 µs.

| Test  | Duration |
| ----- | -------- |
| parse | 0.002 ms |
| part1 | 0.007 ms |
| part2 | 0.007 ms |
