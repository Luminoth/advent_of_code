const PART_1_SCORE: [[usize; 3]; 3] = [
    [3_usize, 6_usize, 0_usize],
    [0_usize, 3_usize, 6_usize],
    [6_usize, 0_usize, 3_usize],
];

fn part1(rounds: impl AsRef<[(usize, usize)]>) {
    let total: usize = rounds
        .as_ref()
        .iter()
        .map(|(a, b)| *b + 1 + PART_1_SCORE[*a][*b])
        .sum();

    assert!(total == 13484);
    println!("Total: {}", total);
}

const PART_2_CHOICE: [[usize; 3]; 3] = [
    [3_usize, 1_usize, 2_usize],
    [1_usize, 2_usize, 3_usize],
    [2_usize, 3_usize, 1_usize],
];

fn part2(rounds: impl AsRef<[(usize, usize)]>) {
    let total: usize = rounds
        .as_ref()
        .iter()
        .map(|(a, b)| *b * 3 + PART_2_CHOICE[*a][*b])
        .sum();

    assert!(total == 13433);
    println!("Total: {}", total);
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

            Some(x.split_once(' ').unwrap())
        })
        .map(|(a, b)| {
            let a = match a {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => unreachable!(),
            };

            let b = match b {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => unreachable!(),
            };

            (a, b)
        })
        .collect::<Vec<_>>();

    part1(&values);
    part2(&values);
}
