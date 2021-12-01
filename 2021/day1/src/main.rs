fn part1(values: impl AsRef<[usize]>) {
    let mut increased = 0;
    for v in values.as_ref().windows(2) {
        if v[1] > v[0] {
            increased += 1;
        }
    }

    assert!(increased == 1754);
    println!("Depth measurement increased {} times", increased);
}

fn part2(values: impl AsRef<[usize]>) {
    let mut prev_sum = 0;
    let mut increased = -1;
    for v in values.as_ref().windows(3) {
        let sum = v.iter().sum();
        if sum > prev_sum {
            increased += 1;
        }
        prev_sum = sum;
    }

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
