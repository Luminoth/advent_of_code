# Advent of Code

* [Advent of Code](https://adventofcode.com/)

## Tips and Tricks

* 2021 has a ton of really clever snippets from folks at Meta that helped solve things
* Iterators and adapters (map, filter, fold) are better optimized than for loops
* HashSets / HashMaps, memoization, look up tables
* Boolean HashMaps can often be done as a bitset if small enough range
* Parallelization is usually a trap, there's an algorithm or pattern somewhere that should be used instead
* nom is great for parsing inputs
* Avoid "arithmetic simulation" bottlenecks
  * Simulating arithmetic computation is usually too slow in AoC problems and very likely the wrong path
  * Frequently an algebraic approach is better
* In Rust, `rem_euclid()` does modulus (% is remainder)
* When generating / checking properties for a vast range of numbers, it is often much faster to generate the numbers with the property and then check if they fall within the given ranges, rather than iterating through every number in the ranges and checking the property.
* Tight loop allocations are obviously bad

## Other Sorts of Solvers

* 2025/day10 part 2 is an ILP (Integer Linear Programming) problem that can be solved with Linear Algebra
  * Use the Z3 solver library to deal with this

## Simple tricks to consider if stuck

* Modulo lowest common multiple (lcm) when needing to maintain a divisor over a set of buckets
  * 2022/day11

## Future thoughts

* A re-usable shortest path trait or something might be really useful for a lot of problems

## Useful code snippets

```
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
```

```
fn get_digit(n: usize, i: u32) -> usize {
    (n / (10_usize).pow(i)) % 10
}
```

```
fn count_digits(mut n: usize) -> u32 {
    // floating point is slower than looping
    //(n as f64).log(10.0).floor() as u32 + 1

    if n == 0 {
        return 1;
    }

    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}
```
