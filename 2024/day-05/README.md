# Advent of Code 2024 Day 05

See the description of the puzzle at <https://adventofcode.com/2024/day/5>.  
Many thanks to Eric Wastl for providing these challenges.

MIT License  
Copyright (c) 2024 Gunter Schmidt  
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Check the order of elements, eventually reorder.

### Coding Highlights

The immidiate reflex is to build a HashMap. This is a good example, where a simple Vector or Array is much faster
because access can be instantly.

A 100x100 matrix is build for the allowed page sequences, where the value is true for allowed page followings,
e.g. 47|53 -> map\[47\]\[53\] = true.

To check, if a page sequence is allowed, only that field needs to be checked. There is no search or HashMap evaluation required.

As a result, this is much faster.

In fact, part 1 run time is 95% parsing the data, this is why it does not at measurable extra time when doing part 1 and 2 in the same function.

---

| Test          | Duration |
| ------------- | -------- |
| part1 HashMap | 0.056 ms |
| part1 Array   | 0.030 ms |
| part2 HashMap | 0.170 ms |
| part2 Array   | 0.054 ms |
| both  Array   | 0.054 ms |

