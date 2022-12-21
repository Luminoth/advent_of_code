use std::cell::RefCell;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_until1},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug, Clone)]
enum MonkeyJobOperation {
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

impl MonkeyJobOperation {
    fn calculate(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Self::Addition(x, y) => {
                let a = monkeys.get(x).unwrap();
                let b = monkeys.get(y).unwrap();
                a.value(monkeys) + b.value(monkeys)
            }
            Self::Subtraction(x, y) => {
                let a = monkeys.get(x).unwrap();
                let b = monkeys.get(y).unwrap();
                a.value(monkeys) - b.value(monkeys)
            }
            Self::Multiplication(x, y) => {
                let a = monkeys.get(x).unwrap();
                let b = monkeys.get(y).unwrap();
                a.value(monkeys) * b.value(monkeys)
            }
            Self::Division(x, y) => {
                let a = monkeys.get(x).unwrap();
                let b = monkeys.get(y).unwrap();
                a.value(monkeys) / b.value(monkeys)
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

#[derive(Debug, Clone)]
enum MonkeyJob {
    Number(i64),
    Operation(MonkeyJobOperation, RefCell<Option<i64>>),
}

impl MonkeyJob {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Self::Number(v) => *v,
            Self::Operation(op, v) => {
                if let Some(v) = *v.borrow() {
                    return v;
                }

                let r = op.calculate(monkeys);
                *v.borrow_mut() = Some(r);
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
        Self::Operation(v, RefCell::new(None))
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

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: MonkeyJob,
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        self.job.value(monkeys)
    }
}

impl From<(String, MonkeyJob)> for Monkey {
    fn from(v: (String, MonkeyJob)) -> Self {
        Self {
            name: v.0,
            job: v.1,
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

fn part1(monkeys: HashMap<String, Monkey>) {
    let rv = monkeys.get("root").unwrap().value(&monkeys);
    assert!(rv == 87457751482938);
    println!("Root value: {}", rv);
}

fn part2(_monkeys: HashMap<String, Monkey>) {}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|x| all_consuming(parse_monkey)(x).finish().unwrap().1)
        .map(|x| (x.name.clone(), x))
        .collect::<HashMap<String, Monkey>>();

    part1(values.clone());
    part2(values);
}
