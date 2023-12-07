use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Card(usize);

impl From<(char, bool)> for Card {
    fn from(v: (char, bool)) -> Self {
        if v.0.is_ascii_digit() {
            return Self(v.0.to_digit(10).unwrap() as usize);
        }

        Self(match v.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if v.1 {
                    1
                } else {
                    11
                }
            }
            'T' => 10,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
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
            match card.cmp(&other.cards[idx]) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
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
    fn get_hand_type(cards: &[Card]) -> HandType {
        let card_counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
            acc.entry(card.0).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|v| *v == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|v| *v == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn get_hand_type_replace(cards: &[Card], idx: usize, card: Card) -> HandType {
        let mut cards = cards.to_owned();
        cards[idx] = card;

        if let Some(idx) = cards.iter().position(|card| card.0 == 1) {
            let mut max = HandType::HighCard;
            for v in 2..=14 {
                if v == 11 {
                    continue;
                }
                max = Self::get_hand_type_replace(&cards, idx, Card(v)).max(max)
            }
            max
        } else {
            Self::get_hand_type(&cards)
        }
    }

    fn new(v: &str, joker: bool) -> Self {
        let parts = v.split_once(' ').unwrap();
        let cards = parts
            .0
            .chars()
            .map(|ch| Card::from((ch, joker)))
            .collect::<Vec<_>>();
        let bid = parts.1.parse::<usize>().unwrap();

        let r#type = if joker {
            // just brute force the dang thing
            if let Some(idx) = cards.iter().position(|card| card.0 == 1) {
                let mut max = HandType::HighCard;
                for v in 2..=14 {
                    if v == 11 {
                        continue;
                    }
                    max = Self::get_hand_type_replace(&cards, idx, Card(v)).max(max)
                }
                max
            } else {
                Self::get_hand_type(&cards)
            }
        } else {
            Self::get_hand_type(&cards)
        };

        Self { r#type, cards, bid }
    }
}

fn part1(mut hands: Vec<Hand>) {
    hands.sort();

    let mut total = 0;
    for (rank, hand) in hands.iter().enumerate() {
        let value = hand.bid * (rank + 1);
        //println!("value of {:?}, rank {} is {}", hand, rank + 1, value);
        total += value;
    }

    assert!(total == 249748283);
    println!("Total: {}", total);
}

fn part2(mut hands: Vec<Hand>) {
    hands.sort();

    let mut total = 0;
    for (rank, hand) in hands.iter().enumerate() {
        let value = hand.bid * (rank + 1);
        //println!("value of {:?}, rank {} is {}", hand, rank + 1, value);
        total += value;
    }

    assert!(total == 248029057);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let hands = input
        .lines()
        .map(|line| Hand::new(line, false))
        .collect::<Vec<_>>();
    part1(hands);

    let hands = input
        .lines()
        .map(|line| Hand::new(line, true))
        .collect::<Vec<_>>();
    part2(hands);
}
