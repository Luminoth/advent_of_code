use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Card(usize);

impl From<char> for Card {
    fn from(v: char) -> Self {
        if v.is_ascii_digit() {
            return Self(v.to_digit(10).unwrap() as usize);
        }

        Self(match v {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, Eq)]
struct Hand {
    r#type: HandType,
    cards: Vec<Card>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.r#type != other.r#type {
            return self.r#type.cmp(&other.r#type);
        }

        for (idx, card) in self.cards.iter().enumerate() {
            // not sure why these reverse :/
            if *card < other.cards[idx] {
                return Ordering::Greater;
            } else if *card > other.cards[idx] {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.cards == other.cards
    }
}

impl Hand {
    fn new(v: &str) -> Self {
        let parts = v.split_once(' ').unwrap();
        let cards = parts.0.chars().map(Card::from).collect::<Vec<_>>();
        let bid = parts.1.parse::<usize>().unwrap();

        let card_counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
            acc.entry(card.0).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        let r#type = if card_counts.len() == 1 {
            // must be 5 of a kind
            HandType::FiveOfAKind
        } else if card_counts.len() == 2 {
            // must be either 4 of a kind or full house
            if card_counts.values().any(|v| *v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if card_counts.len() == 3 {
            // could be 3 of a kind, or 2 pair
            if card_counts.values().any(|v| *v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if card_counts.len() == 4 {
            // must be one pair
            HandType::OnePair
        } else {
            // must be high card
            HandType::HighCard
        };

        Self { r#type, cards, bid }
    }
}

fn part1(hands: &[Hand]) {
    let mut hands = hands.to_owned();
    hands.sort();

    let mut total = 0;
    for (rank, hand) in hands.iter().rev().enumerate() {
        let value = hand.bid * (rank + 1);
        //println!("value of {:?} is {}", hand, value);
        total += value;
    }

    assert!(total == 249748283);
    println!("Total: {}", total);
}

fn part2(_hands: &[Hand]) {
    // not today satan
}

fn main() {
    let input = include_str!("../input.txt");

    let hands = input.lines().map(Hand::new).collect::<Vec<_>>();

    part1(&hands);
    part2(&hands);
}
