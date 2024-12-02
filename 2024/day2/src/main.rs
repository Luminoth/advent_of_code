use std::cmp::Ordering;

// TODO: i think dampening has to try removing both bad values ?

fn is_safe(levels: &Vec<isize>, dampen: bool) -> bool {
    match levels[0].cmp(&levels[1]) {
        Ordering::Less => {
            for window in levels.windows(2).enumerate() {
                let a = window.1[0];
                let b = window.1[1];

                if a >= b {
                    if dampen {
                        let mut levels = levels.clone();
                        levels.remove(window.0 + 1);
                        return is_safe(&levels, false);
                    }
                    return false;
                }

                if (a - b).abs() > 3 {
                    if dampen {
                        let mut levels = levels.clone();
                        levels.remove(window.0 + 1);
                        return is_safe(&levels, false);
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
                        let mut levels = levels.clone();
                        levels.remove(window.0 + 1);
                        return is_safe(&levels, false);
                    }
                    return false;
                }

                if (a - b).abs() > 3 {
                    if dampen {
                        let mut levels = levels.clone();
                        levels.remove(window.0 + 1);
                        return is_safe(&levels, false);
                    }
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
