fn part1(rules: &[(usize, usize)], pages: &[Vec<usize>]) {
    println!("{:?}", rules);
    println!("{:?}", pages);
}

fn main() {
    let input = include_str!("../input.txt").split_once("\n\n").unwrap();

    let rules = input
        .0
        .lines()
        .map(|line| {
            let pages = line.split_once('|').unwrap();
            (pages.0.parse().unwrap(), pages.1.parse().unwrap())
        })
        .collect::<Vec<_>>();
    let pages = input
        .1
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect::<Vec<_>>();

    part1(&rules, &pages);
}
