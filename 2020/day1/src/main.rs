use std::io::{self, BufRead};

const TARGET_SUM: i64 = 2020;

fn part1(expenses: impl AsRef<[i64]>) {
    let expenses = expenses.as_ref();

    for x in 0..expenses.len() {
        for y in x..expenses.len() {
            let a = expenses.get(x).unwrap();
            let b = expenses.get(y).unwrap();

            if a + b == TARGET_SUM {
                println!(
                    "{} + {} = {} and {} * {} = {}",
                    a,
                    b,
                    TARGET_SUM,
                    a,
                    b,
                    a * b
                );
                return;
            }
        }
    }

    panic!("Part 1 found no values found that sum to {}!", TARGET_SUM);
}

fn part2(expenses: impl AsRef<[i64]>) {
    let expenses = expenses.as_ref();

    for x in 0..expenses.len() {
        for y in x..expenses.len() {
            for z in y..expenses.len() {
                let a = expenses.get(x).unwrap();
                let b = expenses.get(y).unwrap();
                let c = expenses.get(z).unwrap();

                if a + b + c == TARGET_SUM {
                    println!(
                        "{} + {} + {} = {} and {} * {} * {} = {}",
                        a,
                        b,
                        c,
                        TARGET_SUM,
                        a,
                        b,
                        c,
                        a * b * c
                    );
                    return;
                }
            }
        }
    }

    panic!("Part 2 found no values found that sum to {}!", TARGET_SUM);
}

fn main() {
    println!("Enter expenses one line at a time:");
    let expenses: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| x.parse::<i64>()) // not sure how to simplify this
        .map(Result::unwrap)
        .collect();

    part1(&expenses);
    part2(&expenses);
}
