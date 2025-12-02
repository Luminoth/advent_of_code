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
* In Rust, `rem_euclid()` does modulus (% is remainder)

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
fn count_digits(n: usize) -> u32 {
    (n as f64).log(10.0).floor() as u32 + 1
}
```
