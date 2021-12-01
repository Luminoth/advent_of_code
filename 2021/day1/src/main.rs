fn part1(values: impl AsRef<[usize]>) {
    let increased: usize = values
        .as_ref()
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();

    assert!(increased == 1754);
    println!("Depth measurement increased {} times", increased);
}

fn part2(values: impl AsRef<[usize]>) {
    let mut prev_sum = 0;
    let increased: usize = values
        .as_ref()
        .windows(3)
        .map(|x| {
            let sum = x.iter().sum();
            let ret = if prev_sum != 0 && sum > prev_sum {
                1
            } else {
                0
            };
            prev_sum = sum;

            ret
        })
        .sum();

    assert!(increased == 1789);
    println!("Depth-sum measurement increased {} times", increased);
}

fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<usize> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if !x.is_empty() {
                Some(x.parse().unwrap())
            } else {
                None
            }
        })
        .collect();

    part1(&values);
    part2(&values);
}
