use std::cmp::Ordering;

fn retry_is_safe(levels: &Vec<isize>, remove: usize) -> bool {
    let mut levels = levels.clone();
    levels.remove(remove);
    is_safe(&levels, false)
}

fn retry_is_safe_naive(levels: &Vec<isize>) -> bool {
    for idx in 0..levels.len() {
        if retry_is_safe(levels, idx) {
            return true;
        }
    }
    false
}

fn is_safe(levels: &Vec<isize>, dampen: bool) -> bool {
    match levels[0].cmp(&levels[1]) {
        Ordering::Less => {
            for window in levels.windows(2).enumerate() {
                let a = window.1[0];
                let b = window.1[1];

                if a >= b {
                    if dampen {
                        return retry_is_safe_naive(&levels);
                    }
                    return false;
                }

                if (a - b).abs() > 3 {
                    if dampen {
                        return retry_is_safe_naive(&levels);
                    }
                    return false;
                }
            }
            true
        }
        Ordering::Greater => {
            for window in levels.windows(2).enumerate() {
                let a = window.1[0];
                let b = window.1[1];

                if a <= b {
                    if dampen {
                        return retry_is_safe_naive(&levels);
                    }
                    return false;
                }

                if (a - b).abs() > 3 {
                    if dampen {
                        return retry_is_safe_naive(&levels);
                    }
                    return false;
                }
            }
            true
        }
        Ordering::Equal => {
            if dampen {
                return retry_is_safe_naive(&levels);
            }
            false
        }
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

    assert!(safe == 544);
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
