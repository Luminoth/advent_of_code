#![allow(clippy::let_and_return)]

// NOTE: this heavily abuses the consistency of the input we're given for parsing
// NOTE: I absolutely had to cheat on the math required for part 2 ... thanks https://www.reddit.com/r/adventofcode/

use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Square,
    Mult(usize),
    Add(usize),
}

impl Operation {
    fn perform(&self, old: usize) -> usize {
        match self {
            Self::Square => {
                let new = old * old;
                #[cfg(feature = "debugmonkeys")]
                println!("    Worry level is multiplied by itself to {}.", new);
                new
            }
            Self::Mult(v) => {
                let new = old * v;
                #[cfg(feature = "debugmonkeys")]
                println!("    Worry level is multiplied by {} to {}.", v, new);
                new
            }
            Self::Add(v) => {
                let new = old + v;
                #[cfg(feature = "debugmonkeys")]
                println!("    Worry level increases by {} to {}.", v, new);
                new
            }
        }
    }
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        let (_, input) = input.split_once('=').unwrap();

        let mut x = input.split_whitespace();
        x.next().unwrap();

        let a = x.next().unwrap().trim();
        match a {
            "*" => {
                let b = x.next().unwrap().trim();
                match b {
                    "old" => Self::Square,
                    _ => Self::Mult(b.parse().unwrap()),
                }
            }
            "+" => {
                let b = x.next().unwrap().trim();
                Self::Add(b.parse().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Test {
    divisible_by: usize,
    true_throw: usize,
    false_throw: usize,
}

impl Test {
    fn test(&self, worry_level: usize) -> usize {
        if worry_level % self.divisible_by == 0 {
            #[cfg(feature = "debugmonkeys")]
            println!(
                "    Current worry level is divisible by {}.",
                self.divisible_by
            );
            self.true_throw
        } else {
            #[cfg(feature = "debugmonkeys")]
            println!(
                "    Current worry level is not divisible by {}.",
                self.divisible_by
            );
            self.false_throw
        }
    }
}

impl From<(&str, &str, &str)> for Test {
    fn from(input: (&str, &str, &str)) -> Self {
        let (_, test) = input.0.split_once(':').unwrap();
        let test = test.split_whitespace().collect::<Vec<_>>();
        let divisible_by = test[2].parse().unwrap();

        let true_throw = input.1.split_whitespace().last().unwrap().parse().unwrap();
        let false_throw = input.2.split_whitespace().last().unwrap().parse().unwrap();

        Self {
            divisible_by,
            true_throw,
            false_throw,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    worry_levels: VecDeque<usize>,
    operation: Operation,
    test: Test,

    inspected_count: usize,
}

impl Monkey {
    fn inspect_items(&mut self, monkeys: impl AsRef<[RefCell<Monkey>]>, lcm: Option<usize>) {
        let mut throws = vec![];
        for worry_level in self.worry_levels.iter_mut() {
            #[cfg(feature = "debugmonkeys")]
            println!(
                "  Monkey inspects an item with a worry level of {}.",
                *worry_level
            );

            // inspect the item
            *worry_level = self.operation.perform(*worry_level);

            // manage worry level
            if let Some(lcm) = lcm {
                *worry_level %= lcm;
                #[cfg(feature = "debugmonkeys")]
                println!("    Managing worry level by {} to {}.", lcm, *worry_level);
            } else {
                *worry_level /= 3;
                #[cfg(feature = "debugmonkeys")]
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    *worry_level
                );
            }

            // test the item and throw it
            // (assuming here we never throw back to ourself)
            let throw = self.test.test(*worry_level);
            #[cfg(feature = "debugmonkeys")]
            println!(
                "    Item with worry level {} is thrown to monkey {}.",
                worry_level, throw
            );
            throws.push(throw);

            self.inspected_count += 1;
        }

        for throw in throws {
            let worry_level = self.worry_levels.pop_front().unwrap();
            monkeys.as_ref()[throw]
                .borrow_mut()
                .worry_levels
                .push_back(worry_level);
        }
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let lines = input.split('\n').collect::<Vec<_>>();
        assert!(lines.len() == 6);

        let (_, worry_levels) = lines[1].split_once(':').unwrap();
        let worry_levels = worry_levels
            .trim()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect::<VecDeque<_>>();

        let (_, operation) = lines[2].split_once(':').unwrap();
        let operation = operation.into();

        let test = (lines[3], lines[4], lines[5]).into();

        Self {
            worry_levels,
            operation,
            test,
            inspected_count: 0,
        }
    }
}

fn simulate(mut monkeys: Vec<RefCell<Monkey>>, rounds: usize, relief: bool) -> usize {
    // for the no-relief simulation, we need the lowest common multiple
    // for all of the monkey tests
    let lcm = if !relief {
        let mut lcm = 1;
        for monkey in &monkeys {
            lcm *= monkey.borrow().test.divisible_by;
        }
        Some(lcm)
    } else {
        None
    };

    let mut round = 0;
    loop {
        if round >= rounds {
            break;
        }

        for (_i, monkey) in monkeys.iter().enumerate() {
            #[cfg(feature = "debugmonkeys")]
            println!("Monkey {}:", _i);
            monkey.borrow_mut().inspect_items(&monkeys, lcm);
        }

        #[cfg(feature = "debugrounds")]
        {
            println!(
                "After round {} the monkeys are holding items with these worry levels:",
                round + 1
            );
            for (i, monkey) in monkeys.iter().enumerate() {
                println!("Monkey {}: {:?}", i, monkey.borrow().worry_levels);
            }
        }

        round += 1;
    }

    #[cfg(feature = "debugrounds")]
    println!("{:?}", monkeys);

    monkeys.sort_by(|a, b| {
        b.borrow()
            .inspected_count
            .partial_cmp(&a.borrow().inspected_count)
            .unwrap()
    });

    monkeys[0].borrow().inspected_count * monkeys[1].borrow().inspected_count
}

fn main() {
    let input = include_str!("../input.txt");

    // TODO: this would probably be a lot better done with nom
    // but I'm still learning my way through that
    // and don't want to take the time to work it out just now
    let values = input
        .split("\n\n")
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(RefCell::new(x.into()))
        })
        .collect::<Vec<_>>();

    let monkey_business = simulate(values.clone(), 20, true);
    assert!(monkey_business == 101436);
    println!("Monkey business: {}", monkey_business);

    let monkey_business = simulate(values, 10000, false);
    assert!(monkey_business == 19754471646);
    println!("Monkey business: {}", monkey_business);
}
