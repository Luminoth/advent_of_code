use std::cmp::Ordering;

fn is_safe(levels: impl AsRef<[isize]>, _dampen: bool) -> bool {
    let levels = levels.as_ref();

    match levels[0].cmp(&levels[1]) {
        Ordering::Less => {
            for window in levels.windows(2) {
                if window[0] >= window[1] {
                    return false;
                }

                if (window[0] - window[1]).abs() > 3 {
                    return false;
                }
            }
            true
        }
        Ordering::Greater => {
            for window in levels.windows(2) {
                if window[0] <= window[1] {
                    return false;
                }

                if (window[0] - window[1]).abs() > 3 {
                    return false;
                }
            }
            true
        }
        Ordering::Equal => false,
    }
}

fn part1(reports: &[Vec<isize>]) {
    let mut safe = 0;

    for report in reports {
        if is_safe(report, false) {
            safe += 1;
        }
    }

    assert!(safe == 502);
    println!("Safe undampened reports: {}", safe);
}

fn part2(reports: &[Vec<isize>]) {
    let mut safe = 0;

    for report in reports {
        if is_safe(report, true) {
            safe += 1;
        }
    }

    //assert!(safe == ???);
    println!("Safe dampened reports: {}", safe);
}

fn main() {
    let input = include_str!("../input.txt");

    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<isize>>>();

    part1(&reports);
    part2(&reports);
}
