fn part1(values: impl AsRef<[usize]>) {
    let max = *values.as_ref().iter().max().unwrap();

    assert!(max == 71502);
    println!("Max calories: {}", max);
}

fn part2(values: impl Into<Vec<usize>>) {
    let mut values = values.into();
    values.sort();
    values.reverse();

    let total = values[0] + values[1] + values[2];

    assert!(total == 208191);
    println!("Top 3 total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut values = vec![0];
    let mut current = values.last_mut().unwrap();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            values.push(0);
            current = values.last_mut().unwrap();
            continue;
        }

        *current += line.parse::<usize>().unwrap();
    }

    part1(&values);
    part2(values);
}
