use std::ops::RangeInclusive;

fn count_digits(n: usize) -> u32 {
    (n as f64).log(10.0).floor() as u32 + 1
}

fn part1<'a>(ranges: impl AsRef<[RangeInclusive<usize>]>) {
    let mut total = 0;

    for range in ranges.as_ref().iter().cloned() {
        for n in range {
            // digits have to repeat twice so must be even number of them
            let digits = count_digits(n);
            if digits % 2 == 1 {
                continue;
            }

            let half_digits = digits / 2;
            let divisor = 10_usize.pow(half_digits);

            let first_half = n / divisor;
            let second_half = n % divisor;

            if first_half == second_half {
                //println!("invalid id={n}");
                total += n;
            }

            /*println!(
                "n={}, digits={}, first={}, second={}",
                n, digits, first_half, second_half
            );*/
        }
    }

    assert!(total == 30608905813);
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
}
