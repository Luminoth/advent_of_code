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
    // the problem here is asking for sliding window sums (A + B + C) > (B + C + D)
    // and I had been tracking that previous sum value for the comparision
    // but a note from https://github.com/zertosh/ that really helped make this simpler:
    // in A + B + C > B + C + D the B and C cancel out from both sides leaving you with just A > D

    let increased: usize = values
        .as_ref()
        .windows(4)
        .map(|x| if x[3] > x[0] { 1 } else { 0 })
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
            if x.is_empty() {
                return None;
            }

            Some(x.parse().unwrap())
        })
        .collect();

    part1(&values);
    part2(&values);
}
