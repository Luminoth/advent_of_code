fn part1(rounds: impl AsRef<[(&'static str, &'static str)]>) {
    let total: usize = rounds
        .as_ref()
        .iter()
        .map(|(a, b)| {
            let choice = match *b {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };

            let result = match *a {
                "A" => match choice {
                    1 => 3,
                    2 => 6,
                    3 => 0,
                    _ => unreachable!(),
                },
                "B" => match choice {
                    1 => 0,
                    2 => 3,
                    3 => 6,
                    _ => unreachable!(),
                },
                "C" => match choice {
                    1 => 6,
                    2 => 0,
                    3 => 3,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            choice + result
        })
        .sum();

    assert!(total == 13484);
    println!("Total: {}", total);
}

fn part2(rounds: impl AsRef<[(&'static str, &'static str)]>) {
    let total: usize = rounds
        .as_ref()
        .iter()
        .map(|(a, b)| {
            let result = match *b {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => unreachable!(),
            };

            let choice = match *a {
                "A" => match result {
                    0 => 3,
                    3 => 1,
                    6 => 2,
                    _ => unreachable!(),
                },
                "B" => match result {
                    0 => 1,
                    3 => 2,
                    6 => 3,
                    _ => unreachable!(),
                },
                "C" => match result {
                    0 => 2,
                    3 => 3,
                    6 => 1,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            choice + result
        })
        .sum();

    assert!(total == 13433);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<(&str, &str)> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.split_once(' ').unwrap())
        })
        .collect();

    part1(&values);
    part2(&values);
}
