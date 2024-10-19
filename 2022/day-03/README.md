# Advent of Code 2023 Day 1
Here a two different solutions are presented to show the impact of the algorithm on the runtime.

The loop version is the common version, where each letter of the first string is searched in the second string.

The array version stores the letter existence in an array, which is faster as each string only needs to be looped once.

part1_loop:  0.0259 ms  
part1_array: 0.0109 ms

part2_loop:  0.0293 ms  
part2_array: 0.0105 ms
