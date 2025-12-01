# Advent of Code

* [Advent of Code](https://adventofcode.com/)

## Tips and Tricks

* HashSets / HashMaps, memoization, look up tables
* Boolean HashMaps can often be done as a bitset if small enough range
* Iterators are often faster than loops
* Parallelization is usually a trap, there's an algorithm or pattern somewhere that should be used instead
* nom is great for parsing inputs
* Avoid "arithmetic simulation" bottlenecks
  * Simulating arithmetic computation is usually too slow in AoC problems and very likely the wrong path
  * Frequently an algebraic approach is better

## Simple tricks to consider if stuck

* Modulo lowest common multiple (lcm) when needing to maintain a divisor over a set of buckets
  * 2022/day11

## Future thoughts

* A re-usable shortest path trait or something might be really useful for a lot of problems
