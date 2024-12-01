use std::collections::HashMap;

fn part1(mut a: Vec<isize>, mut b: Vec<isize>) {
    assert!(a.len() == b.len());

    a.sort();
    b.sort();

    let mut sum = 0;
    for _ in 0..a.len() {
        sum += (a.pop().unwrap() - b.pop().unwrap()).abs();
    }

    assert!(sum == 2742123);
    println!("sum: {}", sum);
}

fn part2(a: &Vec<isize>, b: &Vec<isize>) {
    let mut counts: HashMap<isize, isize> = HashMap::new();
    for v in b {
        *counts.entry(*v).or_default() += 1;
    }

    let mut score = 0;
    for v in a {
        score += *v * counts.get(v).copied().unwrap_or_default();
    }

    assert!(score == 21328497);
    println!("score: {}", score);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input.lines().map(|line| {
        let mut values = line.split_whitespace().map(|v| v.parse().unwrap());
        (values.next().unwrap(), values.next().unwrap())
    });

    // TODO: probably a way to do this directly in the iterator step?
    let mut a = vec![];
    let mut b = vec![];
    for v in values {
        a.push(v.0);
        b.push(v.1);
    }

    part1(a.clone(), b.clone());
    part2(&a, &b);
}
