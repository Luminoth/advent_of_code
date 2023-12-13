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

            if group_idx >= self.groups.len() {
                //println!("overran groups");
                return false;
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
                        if conditions_idx >= self.conditions.len() {
                            //println!("overran conditions");
                            return false;
                        }

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

fn check_arrangements(record: &mut Record) -> usize {
    if let Some(pos) = record
        .conditions
        .iter()
        .position(|x| *x == Condition::Unknown)
    {
        //println!("twiddle unknown at {}", pos);

        record.conditions[pos] = Condition::Operational;
        let mut arrangements = check_arrangements(record);

        record.conditions[pos] = Condition::Damaged;
        arrangements += check_arrangements(record);

        //println!("got {} arrangements after twiddle", arrangements);

        record.conditions[pos] = Condition::Unknown;
        arrangements
    } else if record.is_valid() {
        1
    } else {
        0
    }
}

fn part1(records: &mut [Record]) {
    let mut total = 0;

    for record in records {
        let arrangements = check_arrangements(record);
        println!("got {} arrangements of {:?}", arrangements, record);
        total += arrangements;
    }

    // TODO: getting 32433 which is too high
    // and the number of arrangements per-record on the test data is completely wrong
    // (so probably the validity check is wrong)
    //assert!(total == ???);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");
    let mut records = input.lines().map(Record::from).collect::<Vec<_>>();

    part1(&mut records);
}
