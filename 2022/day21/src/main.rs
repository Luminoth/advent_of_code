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
    Equality(String, String),
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

impl MonkeyJobOperation {
    fn left(&self) -> &String {
        match self {
            Self::Equality(x, _) => x,
            Self::Addition(x, _) => x,
            Self::Subtraction(x, _) => x,
            Self::Multiplication(x, _) => x,
            Self::Division(x, _) => x,
        }
    }

    fn right(&self) -> &String {
        match self {
            Self::Equality(_, y) => y,
            Self::Addition(_, y) => y,
            Self::Subtraction(_, y) => y,
            Self::Multiplication(_, y) => y,
            Self::Division(_, y) => y,
        }
    }

    // returns the operation result and whether the human value was used
    fn calculate(&self, monkeys: &HashMap<String, Monkey>) -> (i64, bool) {
        match self {
            Self::Addition(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                (av + bv, ah || bh)
            }
            Self::Subtraction(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                (av - bv, ah || bh)
            }
            Self::Multiplication(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                (av * bv, ah || bh)
            }
            Self::Division(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                (av / bv, ah || bh)
            }
            _ => unreachable!(),
        }
    }

    fn get_human_value(&self, monkeys: &HashMap<String, Monkey>, result: Option<i64>) -> i64 {
        match self {
            Self::Equality(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                if ah {
                    a.get_human_value(monkeys, Some(bv))
                } else if bh {
                    b.get_human_value(monkeys, Some(av))
                } else {
                    unreachable!()
                }
            }
            Self::Addition(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                // a + b = r
                if ah {
                    // a = r - b
                    a.get_human_value(monkeys, Some(result.unwrap() - bv))
                } else if bh {
                    // b = r - a
                    b.get_human_value(monkeys, Some(result.unwrap() - av))
                } else {
                    unreachable!()
                }
            }
            Self::Subtraction(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                // a - b = r
                if ah {
                    // a = r + b
                    a.get_human_value(monkeys, Some(result.unwrap() + bv))
                } else if bh {
                    // b = -(r - a) = a - r
                    b.get_human_value(monkeys, Some(av - result.unwrap()))
                } else {
                    unreachable!()
                }
            }
            Self::Multiplication(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                // a * b = r
                if ah {
                    // a = r / b
                    a.get_human_value(monkeys, Some(result.unwrap() / bv))
                } else if bh {
                    // b = r / a
                    b.get_human_value(monkeys, Some(result.unwrap() / av))
                } else {
                    unreachable!()
                }
            }
            Self::Division(x, y) => {
                let a = monkeys.get(x).unwrap();
                let (av, ah) = a.value(monkeys);

                let b = monkeys.get(y).unwrap();
                let (bv, bh) = b.value(monkeys);

                // a / b = r
                if ah {
                    // a = r * b
                    a.get_human_value(monkeys, Some(result.unwrap() * bv))
                } else if bh {
                    // b = 1 / (r / a) = a / r
                    b.get_human_value(monkeys, Some(av / result.unwrap()))
                } else {
                    unreachable!()
                }
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
    Operation(MonkeyJobOperation, RefCell<Option<(i64, bool)>>),
}

impl MonkeyJob {
    fn left(&self) -> &String {
        match self {
            Self::Operation(op, _) => op.left(),
            _ => unreachable!(),
        }
    }

    fn right(&self) -> &String {
        match self {
            Self::Operation(op, _) => op.right(),
            _ => unreachable!(),
        }
    }

    // returns the value and whether the human value was used
    fn value(&self, monkeys: &HashMap<String, Monkey>, is_human: bool) -> (i64, bool) {
        match self {
            Self::Number(v) => (*v, is_human),
            Self::Operation(op, v) => {
                if let Some(v) = *v.borrow() {
                    //println!("cache hit!");
                    return v;
                }

                let r = op.calculate(monkeys);

                // only cache values that don't depend on the human value
                if !r.1 {
                    *v.borrow_mut() = Some(r);
                }

                r
            }
        }
    }

    fn get_human_value(
        &self,
        monkeys: &HashMap<String, Monkey>,
        is_human: bool,
        result: Option<i64>,
    ) -> i64 {
        match self {
            Self::Number(v) => {
                if !is_human {
                    *v
                } else {
                    result.unwrap()
                }
            }
            Self::Operation(op, _) => op.get_human_value(monkeys, result),
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

    is_human: bool,
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> (i64, bool) {
        self.job.value(monkeys, self.is_human)
    }

    fn get_human_value(&self, monkeys: &HashMap<String, Monkey>, result: Option<i64>) -> i64 {
        self.job.get_human_value(monkeys, self.is_human, result)
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
    let (rv, _) = monkeys.get("root").unwrap().value(monkeys);
    assert!(rv == 87_457_751_482_938);
    println!("Root value: {}", rv);
}

fn part2(mut monkeys: HashMap<String, Monkey>) {
    monkeys.get_mut("root").unwrap().job = {
        let job = &monkeys.get("root").unwrap().job;
        MonkeyJob::Operation(
            MonkeyJobOperation::Equality(job.left().clone(), job.right().clone()),
            RefCell::new(None),
        )
    };

    let human_value = monkeys.get("root").unwrap().get_human_value(&monkeys, None);
    assert!(human_value == 3_221_245_824_363);
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
    part2(values);
}
