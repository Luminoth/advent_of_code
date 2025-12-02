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
    let mut total = 0;

    for range in ranges.as_ref().iter().cloned() {
        for n in range {
            let s = n.to_string();

            let half_len = s.len() / 2;
            for i in 0..half_len {
                let t = &s[0..=i];
                //println!("check {s} / {t}");

                let a = format!("{}{}", s, t);
                let b = format!("{}{}", t, s);
                if a == b {
                    //println!("invalid id={n} ({a} == {b}, s={s}, t={t})");
                    total += n;
                    break;
                }
            }
        }
    }

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
