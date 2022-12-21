use std::collections::HashMap;
use std::io::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_until1},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};
use parking_lot::RwLock;
use rayon::prelude::*;

#[derive(Debug)]
enum MonkeyJobOperation {
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

impl MonkeyJobOperation {
    // returns test result and whether the human override was used
    fn calculate(
        &self,
        monkeys: &HashMap<String, Monkey>,
        human_override: Option<i64>,
    ) -> (i64, bool) {
        match self {
            Self::Addition(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av + bv, ah || bh)
            }
            Self::Subtraction(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av - bv, ah || bh)
            }
            Self::Multiplication(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av * bv, ah || bh)
            }
            Self::Division(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av / bv, ah || bh)
            }
        }
    }

    // returns test result and whether the human override was used
    fn test(&self, monkeys: &HashMap<String, Monkey>, human_override: Option<i64>) -> (bool, bool) {
        match self {
            Self::Addition(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av == bv, ah || bh)
            }
            Self::Subtraction(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av == bv, ah || bh)
            }
            Self::Multiplication(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av == bv, ah || bh)
            }
            Self::Division(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys, human_override);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys, human_override);
                (av == bv, ah || bh)
            }
        }
    }
}

// TODO: not sure why take_till1 can't use is_space and is_newline :(

fn parse_monkey_job_addition(input: &str) -> IResult<&str, MonkeyJobOperation> {
    map(
        separated_pair(
            take_till1(|c| c == ' '),
            tag::<&str, &str, Error<_>>(" + "),
            take_till1(|c| c == '\n'),
        ),
        |(x, y)| MonkeyJobOperation::Addition(x.to_owned(), y.to_owned()),
    )(input)
}

fn parse_monkey_job_subtraction(input: &str) -> IResult<&str, MonkeyJobOperation> {
    map(
        separated_pair(
            take_till1(|c| c == ' '),
            tag::<&str, &str, Error<_>>(" - "),
            take_till1(|c| c == '\n'),
        ),
        |(x, y)| MonkeyJobOperation::Subtraction(x.to_owned(), y.to_owned()),
    )(input)
}

fn parse_monkey_job_multiplication(input: &str) -> IResult<&str, MonkeyJobOperation> {
    map(
        separated_pair(
            take_till1(|c| c == ' '),
            tag::<&str, &str, Error<_>>(" * "),
            take_till1(|c| c == '\n'),
        ),
        |(x, y)| MonkeyJobOperation::Multiplication(x.to_owned(), y.to_owned()),
    )(input)
}

fn parse_monkey_job_division(input: &str) -> IResult<&str, MonkeyJobOperation> {
    map(
        separated_pair(
            take_till1(|c| c == ' '),
            tag::<&str, &str, Error<_>>(" / "),
            take_till1(|c| c == '\n'),
        ),
        |(x, y)| MonkeyJobOperation::Division(x.to_owned(), y.to_owned()),
    )(input)
}

fn parse_monkey_job_operation(input: &str) -> IResult<&str, MonkeyJobOperation> {
    alt((
        parse_monkey_job_addition,
        parse_monkey_job_subtraction,
        parse_monkey_job_multiplication,
        parse_monkey_job_division,
    ))(input)
}

#[derive(Debug)]
enum MonkeyJob {
    Number(i64),
    Operation(MonkeyJobOperation, RwLock<Option<(i64, bool)>>),
}

impl MonkeyJob {
    // returns the value and whether the human override was used
    fn value(
        &self,
        monkeys: &HashMap<String, Monkey>,
        is_human: bool,
        human_override: Option<i64>,
    ) -> (i64, bool) {
        match self {
            Self::Number(v) => {
                if is_human {
                    if let Some(human_override) = human_override {
                        return (human_override, true);
                    }
                }
                (*v, false)
            }
            Self::Operation(op, v) => {
                if let Some(v) = *v.write() {
                    //println!("cache hit!");
                    return v;
                }

                let r = op.calculate(monkeys, human_override);

                // only update the cache if the human value
                // wasn't used and the cache hasn't be set yet
                if !r.1 && v.read().is_none() {
                    *v.write() = Some(r);
                }

                r
            }
        }
    }
}

impl From<i64> for MonkeyJob {
    fn from(v: i64) -> Self {
        Self::Number(v)
    }
}

impl From<MonkeyJobOperation> for MonkeyJob {
    fn from(v: MonkeyJobOperation) -> Self {
        Self::Operation(v, RwLock::new(None))
    }
}

fn parse_monkey_job_number(input: &str) -> IResult<&str, MonkeyJob> {
    map(map_res(digit1, str::parse::<i64>), Into::into)(input)
}

fn parse_monkey_job(input: &str) -> IResult<&str, MonkeyJob> {
    alt((
        parse_monkey_job_number,
        map(parse_monkey_job_operation, Into::into),
    ))(input)
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: MonkeyJob,

    is_human: bool,
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>, human_override: Option<i64>) -> (i64, bool) {
        self.job.value(monkeys, self.is_human, human_override)
    }

    fn test(&self, monkeys: &HashMap<String, Monkey>, human_override: Option<i64>) -> (bool, bool) {
        match &self.job {
            MonkeyJob::Operation(op, _) => op.test(monkeys, human_override),
            _ => unreachable!(),
        }
    }
}

impl From<(String, MonkeyJob)> for Monkey {
    fn from(v: (String, MonkeyJob)) -> Self {
        let is_human = v.0 == "humn";
        Self {
            name: v.0,
            job: v.1,
            is_human,
        }
    }
}

fn parse_monkey_name(input: &str) -> IResult<&str, String> {
    map(take_until1(":"), Into::into)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        separated_pair(parse_monkey_name, tag(": "), parse_monkey_job),
        Into::into,
    )(input)
}

fn part1(monkeys: &HashMap<String, Monkey>) {
    let (rv, _) = monkeys.get("root").unwrap().value(&monkeys, None);
    assert!(rv == 87_457_751_482_938);
    println!("Root value: {}", rv);
}

fn part2(monkeys: &HashMap<String, Monkey>) {
    let human_value = RwLock::new(None);

    // prime the cache (0 is not the answer so this is legit)
    // do this so that we don't need to lock when parallelizing
    monkeys.get("root").unwrap().test(&monkeys, Some(0));

    (100_000_000..1_000_000_000_000)
        .into_par_iter()
        .find_first(|&i| {
            if i % 100_000_000_000 == 0 {
                print!("x");
                std::io::stdout().flush().unwrap();
            } else if i % 10_000_000_000 == 0 {
                print!("@");
                std::io::stdout().flush().unwrap();
            } else if i % 1_000_000_000 == 0 {
                print!("#");
                std::io::stdout().flush().unwrap();
            } else if i % 100_000_000 == 0 {
                print!("-");
                std::io::stdout().flush().unwrap();
            } else if i % 1_000_000 == 0 {
                print!(".");
                std::io::stdout().flush().unwrap();
            }

            let (passed, _) = monkeys.get("root").unwrap().test(&monkeys, Some(i));
            if passed {
                *human_value.write() = Some(i);
                return true;
            }

            false
        });
    println!();

    let human_value = human_value.read().unwrap();
    //assert!(human_value == 87457751482938);
    println!("Human value: {}", human_value);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|x| all_consuming(parse_monkey)(x).finish().unwrap().1)
        .map(|x| (x.name.clone(), x))
        .collect::<HashMap<String, Monkey>>();

    part1(&values);
    part2(&values);
}
