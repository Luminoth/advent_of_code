use std::ops::RangeInclusive;

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

// TODO: AI says this would be faster to generate the possible numbers
// and then check if they fall inside the given ranges
fn part1(ranges: impl AsRef<[RangeInclusive<usize>]>) {
    let total: usize = ranges
        .as_ref()
        .iter()
        .flat_map(|r| r.clone())
        .filter(|&n| {
            let digits = count_digits(n);
            if digits % 2 == 1 {
                return false;
            }

            let half_digits = digits / 2;
            let divisor = 10_usize.pow(half_digits);

            let first_half = n / divisor;
            let second_half = n % divisor;

            /*println!(
                "n={}, digits={}, first={}, second={}",
                n, digits, first_half, second_half
            );*/

            if first_half == second_half {
                //println!("invalid id={n}");
                return true;
            }
            false
        })
        .sum();

    assert!(total == 30608905813);
    println!("Total: {}", total);
}

// solution here is sort of based on https://leetcode.com/problems/greatest-common-divisor-of-strings
// we can find repeating patterns in strings by checking if concatenating them in both orders is the same
fn part2<'a>(ranges: impl AsRef<[RangeInclusive<usize>]>) {
    let total: usize = ranges
        .as_ref()
        .iter()
        .flat_map(|r| r.clone())
        .filter(|&n| {
            // TODO: this allocation is bad
            let s = n.to_string();
            let len = s.len();
            let half_len = len / 2;

            for i in 1..=half_len {
                // only need to check divisors of the string length
                if len % i != 0 {
                    continue;
                }

                let t = &s[0..i];
                //println!("check {s} / {t}");

                // TODO: these allocations are bad
                // check for divisibility (repeating patterns)
                let a = format!("{}{}", s, t);
                let b = format!("{}{}", t, s);
                if a == b {
                    //println!("invalid id={n} ({a} == {b}, s={s}, t={t})");
                    return true;
                }
            }
            false
        })
        .sum();

    assert!(total == 31898925685);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let ranges = input
        .trim()
        .split(',')
        .map(|range| {
            let v = range.split_once('-').unwrap();
            let start = v.0.parse::<usize>().unwrap();
            let end = v.1.parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    part1(&ranges);
    part2(&ranges);
}
