use std::ops::RangeInclusive;

fn part1(fresh_ids: impl AsRef<[RangeInclusive<usize>]>, available_ids: impl AsRef<[usize]>) {
    let fresh_ids = fresh_ids.as_ref();
    let available_ids = available_ids.as_ref();

    let fresh_count = available_ids
        .iter()
        .filter(|available_id| fresh_ids.iter().any(|range| range.contains(available_id)))
        .count();

    assert!(fresh_count == 770);
    println!("Fresh count: {}", fresh_count);
}

fn main() {
    let input = include_str!("../input.txt");

    let (fresh_ids, available_ids) = input.trim().split_once("\n\n").unwrap();

    let fresh_ids = fresh_ids
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let available_ids = available_ids
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    part1(&fresh_ids, &available_ids);
}
