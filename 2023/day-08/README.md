# Advent of Code 2023 Day 8

See the description of the puzzle at <https://adventofcode.com/2023/day/8>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
### The Problem

Part 1: Follow a path by using Left and Right instructions.

Part 2: Follow multiple paths until all reach the end in the same step.

### Algorithmic Highlights

Part 2 would have a very long runtime if the loop would be done until all paths end simultaneously.
Instead each path is followed only once till the end, because then it will repeat itself.
This allows the calculation of the result by using the least common dominator.

E.g. path length 2,3,9 = product 54, but 18 would be the first simultaneous end.
2 and 3 have 6 as the least common dominator
6 and 9 have 18 as the least common dominator

### Coding Highlights

This puzzle calls for a HashMap. In my experience the [hashbrown HashMap](https://docs.rs/hashbrown/latest/hashbrown/) is significantly faster than HashMap and also faster than [fnv](https://docs.rs/fnv/latest/fnv/) and [fxhasher](https://docs.rs/fxhash/latest/fxhash/).  
I am using LEFT = 0 and RIGHT = 1 instead of an enum, because this allows direct access as index.

As always, replacing the HashMap with an array can speed up the calculations by around 30 percent.

Very interesting was the fact the the nested loop has a significant role in the runtime by deciding
which loop is outside and which inside.

The lookup of some data in a vector takes O(1), but this varies greatly. Larger tables have a higher lookup time than smaller. Assume you have a nested loop where one list has 10, the other list 10,000 elements. These are in total 10 * 10,000 = 100,000 lookups. Lets assume a lookup in a vector with 10 elements takes 1 ns and a lookup with 10,000 elements 5 ns. 
Alt 1: 10 * 1 ns + (10 * 10,000) * 5 ns = 500,010 ns
Alt 2: 10,000 * 5 ns + (10 * 10,000) * 1 ns = 150,000 ns
Make sure the shorter list is the inner loop!

Array and casting
The key only needs to be u8 but needs to be casted to usize for array access. Using usize requires 8x more stack space!!!
In my tests the speed was:  
* usize: 0.371 ms  
* u32: 0.320 ms  
* u16: 0.306 ms
* u8: 0.299 ms / (0.620 ms but this seems a criterion issue)


---

| Test         | Description                   | Duration |
| ------------ | ----------------------------- | -------- |
| part1:       | HashMap                       | 0.145 ms |
| part1_array: | Array                         | 0.048 ms |
| part2_v1:    | HashMap with nested loop:<br>simul. keys outer, directions inner| 0.932 ms |
| part2_v2:    | HashMap with nested loop:<br>directions outer, simul. keys inner | 0.430 ms |
| part2_array: | Array                         | 0.300 ms | 
| part2_array<br> 16-bit key: |                | 0.156 ms | 

