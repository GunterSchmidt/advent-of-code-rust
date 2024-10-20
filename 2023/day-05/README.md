# Advent of Code 2023 Day 5

Part 2 was quite challenging. The number ranges made it impossible to just brute force
calculate like in part 1.

So the algorithm must be more efficient. Since a whole range is calculated, but only the
minimum is requested, all numbers above the minimum do not need to be calculated unless
a different branch is entered.

Also a binary search makes the lookup a bit faster.

The total running time of only 0.040 ms makes it very fast compared to other solutions.

part1: 0.032 ms  
part2: 0.038 ms  
