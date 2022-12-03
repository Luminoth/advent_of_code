fn priority(a: char) -> usize {
    if a.is_uppercase() {
        a as usize - 64 + 26
    } else {
        a as usize - 96
    }
}

fn part1(values: impl AsRef<[&'static str]>) {
    let values: Vec<(&str, &str)> = values
        .as_ref()
        .iter()
        .map(|x| {
            let len = x.len();
            assert!(len % 2 == 0);

            x.split_at(x.len() / 2)
        })
        .collect();

    let mut total = 0;
    for value in values {
        let v = value
            .0
            .chars()
            .find(|&a| value.1.chars().any(|b| a == b))
            .unwrap();
        total += priority(v);
    }

    assert!(total == 7821);
    println!("Total priority: {}", total);
}

fn part2(values: impl AsRef<[&'static str]>) {
    let mut total = 0;
    for group in values.as_ref().chunks(3) {
        let v = group[0]
            .chars()
            .find(|&a| {
                group[1]
                    .chars()
                    .any(|b| a == b && group[2].chars().any(|c| b == c))
            })
            .unwrap();
        total += priority(v);
    }

    assert!(total == 2752);
    println!("Total priority: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<&str> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x)
        })
        .collect();

    part1(&values);
    part2(&values);
}
