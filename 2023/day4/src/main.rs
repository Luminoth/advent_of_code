use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl From<&str> for Card {
    fn from(v: &str) -> Self {
        let re =
            Regex::new(r"Card\s+(?<id>\d+):\s+(?<winning_numbers>.+)\|(?<numbers>.+)").unwrap();
        let caps = re.captures(v).unwrap();

        // this is probably all doable in the regex :shrug:
        let winning_numbers = caps["winning_numbers"]
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();
        let numbers = caps["numbers"]
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();

        Self {
            winning_numbers,
            numbers,
        }
    }
}

impl Card {
    fn get_success_count(&self) -> usize {
        let mut count = 0;
        for n in &self.numbers {
            if self.winning_numbers.contains(n) {
                count += 1;
            }
        }
        count
    }
}

fn part1(cards: &Vec<Card>) {
    let mut sum = 0;

    for card in cards {
        let success_count = card.get_success_count();
        let value = match success_count {
            0 => 0,
            _ => 2_usize.pow(success_count as u32 - 1),
        };
        //println!("value: {}", value);
        sum += value;
    }

    assert!(sum == 22488);
    println!("Sum: {}", sum);
}

fn part2(cards: &[Card]) {
    let mut collection = cards
        .iter()
        .map(|card| (card.clone(), 1))
        .collect::<HashMap<_, _>>();

    for (idx, card) in cards.iter().enumerate() {
        let count = *collection.get(card).unwrap();
        let successes = card.get_success_count();

        let start = idx + 1;
        let end = (start + successes).min(cards.len());
        for n in cards.iter().take(end).skip(start) {
            *collection.get_mut(n).unwrap() += count;
        }
    }

    let sum: usize = collection.values().sum();

    assert!(sum == 7013204);
    println!("Sum: {}", sum);
}

fn main() {
    let input = include_str!("../input.txt");

    let cards = input.lines().map(Card::from).collect::<Vec<_>>();

    part1(&cards);
    part2(&cards);
}
