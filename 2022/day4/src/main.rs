// a note from https://fasterthanli.me/series/advent-of-code-2022/part-4
// ranges in Rust have a contains() method (std::ops::RangeInclusive)

fn part1(values: impl AsRef<[((usize, usize), (usize, usize))]>) {
    let mut total = 0;
    for value in values.as_ref() {
        if value.0 .0 <= value.1 .0 && value.0 .1 >= value.1 .1
            || value.1 .0 <= value.0 .0 && value.1 .1 >= value.0 .1
        {
            total += 1;
        }
    }

    assert!(total == 532);
    println!("Total completely overlapping assignments: {}", total);
}

fn part2(values: impl AsRef<[((usize, usize), (usize, usize))]>) {
    let mut total = 0;
    for value in values.as_ref() {
        if value.0 .0 <= value.1 .0 && value.0 .1 >= value.1 .0
            || value.1 .0 <= value.0 .0 && value.1 .1 >= value.0 .0
        {
            total += 1;
        }
    }

    assert!(total == 854);
    println!("Total overlapping assignments: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let pairs = x.split_once(',').unwrap();
            let a = pairs.0.split_once('-').unwrap();
            let b = pairs.1.split_once('-').unwrap();
            Some((
                (a.0.parse::<usize>().unwrap(), a.1.parse::<usize>().unwrap()),
                (b.0.parse::<usize>().unwrap(), b.1.parse::<usize>().unwrap()),
            ))
        })
        .collect::<Vec<_>>();

    part1(&values);
    part2(&values);
}
