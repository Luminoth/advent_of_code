use std::collections::HashMap;

// solution here largely taken from https://github.com/3ach

fn step(
    pairs: HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut result = HashMap::new();
    for (pair, count) in &pairs {
        let ch = rules.get(pair).unwrap();

        let a = (pair.0, *ch);
        let entry = result.entry(a).or_insert(0);
        *entry += *count;

        let b = (*ch, pair.1);
        let entry = result.entry(b).or_insert(0);
        *entry += *count;
    }
    result
}

fn run(template: impl AsRef<str>, rules: &HashMap<(char, char), char>, steps: usize) -> usize {
    // get the initial count of pairs
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for window in template.as_ref().as_bytes().windows(2) {
        let pair = (window[0] as char, window[1] as char);
        let entry = pairs.entry(pair).or_insert(0);
        *entry += 1;
    }

    for _ in 0..steps {
        pairs = step(pairs, rules);
    }

    // count the characters
    let mut counts: HashMap<char, usize> = HashMap::new();
    for (pair, count) in &pairs {
        let entry = counts.entry(pair.0).or_insert(0);
        *entry += *count;

        let entry = counts.entry(pair.1).or_insert(0);
        *entry += *count;
    }

    // bump the first and last template characters
    *counts
        .entry(template.as_ref().chars().next().unwrap())
        .or_insert(0) += 1;
    *counts
        .entry(template.as_ref().chars().last().unwrap())
        .or_insert(0) += 1;

    // account for doubling up on everything
    for count in counts.values_mut() {
        *count /= 2;
    }

    // find the min / max counts
    let (min, max) = counts
        .values()
        .fold((usize::MAX, 0_usize), |(min, max), &count| {
            (count.min(min), count.max(max))
        });
    max - min
}

fn main() {
    let input = include_str!("../input.txt");

    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(char, char), char> = rules
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let (x, y) = x.split_once("->").unwrap();
            let x = x.trim();
            let y = y.trim();
            Some((
                (x.chars().next().unwrap(), x.chars().nth(1).unwrap()),
                y.chars().next().unwrap(),
            ))
        })
        .collect();

    let result = run(&template, &rules, 10);
    assert!(result == 2549);
    println!("Result after 10 steps: {}", result);

    let result = run(&template, &rules, 40);
    assert!(result == 2516901104210);
    println!("Result after 40 steps: {}", result);
}
