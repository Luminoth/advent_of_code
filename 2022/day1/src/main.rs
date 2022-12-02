fn part1(values: impl AsRef<[usize]>) {
    let max = *values.as_ref().iter().max().unwrap();

    assert!(max == 71502);
    println!("Max calories: {}", max);
}

fn part2(values: impl Into<Vec<usize>>) {
    let mut values = values.into();
    values.sort();
    values.reverse();

    let total: usize = values.iter().take(3).sum();

    assert!(total == 208191);
    println!("Top 3 total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>()
        .split(|x| x.is_none())
        .map(|x| x.iter().map(|x| x.unwrap()).sum::<usize>())
        .collect::<Vec<_>>();

    part1(&values);
    part2(values);
}
