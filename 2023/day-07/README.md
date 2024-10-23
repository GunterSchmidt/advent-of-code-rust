# Advent of Code 2023 Day 7

This puzzle has many options to calculate the rank of the cards. 
I chose an array approach which was significantly faster.

Normally I would use an Enum for the Card, but this also took 10% of the time.

At the end all the hands must be sorted which takes 50% of the time.

---

part1_fast:  0.099 ms  
part1_enum:  0.109 ms  
part1_parallel_sort: 0.100 ms  

part2_fast:  0.104 ms  
part2_enum:  0.119 ms  

