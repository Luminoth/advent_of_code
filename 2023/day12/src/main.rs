#![allow(dead_code)]

use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString)]
enum Condition {
    #[strum(serialize = ".")]
    Operational,

    #[strum(serialize = "#")]
    Damaged,

    #[strum(serialize = "?")]
    Unknown,
}

#[derive(Debug, Clone)]
struct Record {
    conditions: Vec<Condition>,
    groups: Vec<usize>,
}

impl From<&str> for Record {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"(?<conditions>(\?|\.|#)+) (?<groups>.+)").unwrap();
        let caps = re.captures(v).unwrap();

        let conditions = caps["conditions"]
            .chars()
            .map(|ch| Condition::from_str(ch.to_string().as_str()).unwrap())
            .collect::<Vec<_>>();
        let groups = caps["groups"]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();

        Self { conditions, groups }
    }
}

impl Record {
    fn is_valid(&self) -> bool {
        let mut conditions_idx = 0;
        let mut group_idx = 0;
        loop {
            if conditions_idx >= self.conditions.len() {
                break;
            }

            /*println!(
                "checking condition {}: {:?}",
                conditions_idx, self.conditions[conditions_idx]
            );*/
            match self.conditions[conditions_idx] {
                Condition::Operational => {
                    conditions_idx += 1;
                    continue;
                }
                Condition::Unknown => return false,
                Condition::Damaged => {
                    //println!("checking group {} ({})", group_idx, self.groups[group_idx]);
                    for _ in 1..self.groups[group_idx] {
                        conditions_idx += 1;

                        /*println!(
                            "checking group {} condition {}: {:?}",
                            group_idx, conditions_idx, self.conditions[conditions_idx]
                        );*/
                        if self.conditions[conditions_idx] != Condition::Damaged {
                            return false;
                        }
                    }
                    conditions_idx += 1;
                    group_idx += 1;
                }
            }
        }

        true
    }
}

fn part1(records: &[Record]) {
    let total = 0;

    for _record in records {
        // TODO:
    }

    //assert!(total == ???);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let records = input.lines().map(Record::from).collect::<Vec<_>>();

    part1(&records);
}
