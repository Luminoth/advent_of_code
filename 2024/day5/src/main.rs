use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct UpdateRule {
    before: HashSet<usize>,
    after: HashSet<usize>,
}

#[derive(Debug, Default)]
struct UpdateRules {
    pages: HashMap<usize, UpdateRule>,
}

fn part1(rules: &UpdateRules, updates: &[Vec<usize>]) {
    let mut total = 0;
    for update in updates {
        // TODO: not sure why but a loop label break in here isn't working
        let mut fail = false;

        for (idx, page) in update.iter().enumerate() {
            let rule = rules.pages.get(page).unwrap();
            for i in idx..update.len() {
                if rule.before.contains(&update[i]) {
                    fail = true;
                    break;
                }
            }

            if fail {
                break;
            }

            for i in (idx..0).rev() {
                if rule.after.contains(&update[i]) {
                    fail = true;
                    break;
                }
            }

            if fail {
                break;
            }
        }

        if !fail {
            let mid = update[update.len() / 2];
            total += mid;
        }
    }

    assert!(total == 5452);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut rules = UpdateRules::default();
    input
        .0
        .lines()
        .map(|line| {
            let pages = line.split_once('|').unwrap();
            (pages.0.parse().unwrap(), pages.1.parse().unwrap())
        })
        .for_each(|(a, b)| {
            rules.pages.entry(a).or_default().after.insert(b);
            rules.pages.entry(b).or_default().before.insert(a);
        });

    let updates = input
        .1
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect::<Vec<_>>();

    part1(&rules, &updates);
}
