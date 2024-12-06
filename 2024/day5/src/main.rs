use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct UpdateRule {
    before: HashSet<usize>,
    after: HashSet<usize>,
}

#[derive(Debug, Default)]
struct UpdateRules {
    rules: HashMap<usize, UpdateRule>,
}

impl UpdateRules {
    fn sort(&self, update: &mut [usize]) {
        update.sort_by(|a, b| {
            let rule = self.rules.get(a).unwrap();

            if rule.before.contains(b) {
                return std::cmp::Ordering::Greater;
            }

            if rule.after.contains(b) {
                return std::cmp::Ordering::Less;
            }

            std::cmp::Ordering::Equal
        });
    }
}

fn part1(rules: &UpdateRules, updates: &[Vec<usize>]) {
    let mut total = 0;
    for update in updates {
        // TODO: not sure why but a loop label break in here isn't working
        let mut fail = false;

        for (idx, page) in update.iter().enumerate() {
            let rule = rules.rules.get(page).unwrap();
            for v in update.iter().skip(idx) {
                if rule.before.contains(v) {
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

fn part2(rules: &UpdateRules, mut updates: Vec<Vec<usize>>) {
    let mut total = 0;
    for update in updates.iter_mut() {
        // TODO: not sure why but a loop label break in here isn't working
        let mut fail = false;

        for idx in 0..update.len() {
            let page = update[idx];

            let rule = rules.rules.get(&page).unwrap();
            for i in idx..update.len() {
                if rule.before.contains(&update[i]) {
                    rules.sort(update);
                    fail = true;
                    break;
                }
            }

            if fail {
                break;
            }

            for i in (idx..0).rev() {
                if rule.after.contains(&update[i]) {
                    rules.sort(update);
                    fail = true;
                    break;
                }
            }

            if fail {
                break;
            }
        }

        if fail {
            let mid = update[update.len() / 2];
            total += mid;
        }
    }

    assert!(total == 4598);
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
            rules.rules.entry(a).or_default().after.insert(b);
            rules.rules.entry(b).or_default().before.insert(a);
        });

    let updates = input
        .1
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect::<Vec<_>>();

    part1(&rules, &updates);
    part2(&rules, updates);
}
