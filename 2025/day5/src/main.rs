use std::ops::RangeInclusive;

fn part1(fresh_id_ranges: impl AsRef<[RangeInclusive<usize>]>, available_ids: impl AsRef<[usize]>) {
    let fresh_id_ranges = fresh_id_ranges.as_ref();
    let available_ids = available_ids.as_ref();

    let fresh_count = available_ids
        .iter()
        .filter(|available_id| {
            fresh_id_ranges
                .iter()
                .any(|range| range.contains(available_id))
        })
        .count();

    assert!(fresh_count == 770);
    println!("Fresh count: {}", fresh_count);
}

fn combine(fresh_id_ranges: &mut Vec<RangeInclusive<usize>>) -> bool {
    for i in 0..fresh_id_ranges.len() - 1 {
        for j in i + 1..fresh_id_ranges.len() {
            let b = fresh_id_ranges[j].clone();
            let a = fresh_id_ranges.get_mut(i).unwrap();

            // ranges don't overlap
            if a.start() > b.end() || b.start() > a.end() {
                continue;
            }

            // ranges do overlap
            let start = a.start().min(b.start());
            let end = a.end().max(b.end());

            *a = *start..=*end;
            fresh_id_ranges.swap_remove(j);
            return true;
        }
    }

    false
}

fn part2(mut fresh_id_ranges: Vec<RangeInclusive<usize>>) {
    while combine(&mut fresh_id_ranges) {}

    let total: usize = fresh_id_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    assert!(total == 357674099117260);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let (fresh_id_ranges, available_ids) = input.trim().split_once("\n\n").unwrap();

    let fresh_id_ranges = fresh_id_ranges
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

    part1(&fresh_id_ranges, &available_ids);
    part2(fresh_id_ranges);
}
