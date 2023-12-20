use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Clone, strum::EnumString)]
enum Destination {
    #[strum(serialize = "A")]
    Accept,

    #[strum(serialize = "R")]
    Reject,

    #[strum(default)]
    Workflow(String),
}

#[derive(Debug, strum::EnumString)]
enum Operand {
    #[strum(serialize = "<")]
    LessThan,

    #[strum(serialize = ">")]
    GreaterThan,
}

#[derive(Debug)]
struct Condition {
    rating: char,
    operand: Operand,
    value: usize,
}

impl From<&str> for Condition {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"(?<rating>x|m|a|s)(?<operand><|>)(?<value>\d+)").unwrap();
        let caps = re.captures(v).unwrap();

        let rating = caps["rating"].chars().next().unwrap();
        let operand = Operand::from_str(&caps["operand"]).unwrap();
        let value = caps["value"].parse::<usize>().unwrap();

        Self {
            rating,
            operand,
            value,
        }
    }
}

impl Condition {
    fn process_part(&self, part: &Part) -> bool {
        let v = match self.rating {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!(),
        };

        match self.operand {
            Operand::LessThan => v < self.value,
            Operand::GreaterThan => v > self.value,
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

impl From<&str> for Rule {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"((?<condition>.*):)?(?<destination>.+)").unwrap();
        let caps = re.captures(v).unwrap();

        let condition = caps
            .name("condition")
            .map(|condition| Condition::from(condition.as_str()));
        let destination = Destination::from_str(&caps["destination"]).unwrap();

        Self {
            condition,
            destination,
        }
    }
}

impl Rule {
    fn process_part(&self, part: &Part) -> Option<Destination> {
        if let Some(condition) = &self.condition {
            if condition.process_part(part) {
                Some(self.destination.clone())
            } else {
                None
            }
        } else {
            Some(self.destination.clone())
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"(?<name>.+)\{(?<rules>.+)\}").unwrap();
        let caps = re.captures(v).unwrap();

        let name = caps["name"].to_owned();
        let rules = caps["rules"].split(',').map(Rule::from).collect::<Vec<_>>();

        Self { name, rules }
    }
}

impl Workflow {
    fn process_part(&self, part: &Part) -> Destination {
        for rule in &self.rules {
            let destination = rule.process_part(part);
            if let Some(destination) = destination {
                return destination;
            }
        }

        unreachable!()
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        let caps = re.captures(v).unwrap();

        let x = caps["x"].parse::<usize>().unwrap();
        let m = caps["m"].parse::<usize>().unwrap();
        let a = caps["a"].parse::<usize>().unwrap();
        let s = caps["s"].parse::<usize>().unwrap();

        Self { x, m, a, s }
    }
}

impl Part {
    fn get_value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &[Part]) {
    let mut accepted = 0;

    for part in parts {
        //println!("checking part {:?}", part);
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let destination = workflow.process_part(part);
            match destination {
                Destination::Accept => {
                    //println!("accepted part: {:?}", part);
                    accepted += part.get_value();
                    break;
                }
                Destination::Reject => {
                    break;
                }
                Destination::Workflow(name) => {
                    //println!("passing to workflow: {}", name);
                    workflow = workflows.get(&name).unwrap();
                }
            }
        }
    }

    assert!(accepted == 489392);
    println!("Accepted: {}", accepted);
}

fn part2(_workflows: &HashMap<String, Workflow>) {
    // TODO: this is now a math problem
}

fn main() {
    let input = include_str!("../input.txt");

    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|workflow| {
            let workflow = Workflow::from(workflow);
            (workflow.name.clone(), workflow)
        })
        .collect::<HashMap<_, _>>();

    let parts: Vec<Part> = parts.lines().map(Part::from).collect::<Vec<_>>();

    part1(&workflows, &parts);
    part2(&workflows);
}
