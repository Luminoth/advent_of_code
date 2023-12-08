use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, strum::EnumString)]
enum Direction {
    #[strum(serialize = "L")]
    Left,

    #[strum(serialize = "R")]
    Right,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"(?<name>.+) = \((?<left>.+), (?<right>.+)\)").unwrap();
        let caps = re.captures(v).unwrap();

        Self {
            name: caps["name"].to_owned(),
            left: caps["left"].to_owned(),
            right: caps["right"].to_owned(),
        }
    }
}

impl Node {
    fn is_start(&self) -> bool {
        self.name.ends_with('A')
    }

    fn is_end(&self, full: bool) -> bool {
        if full {
            self.name == "ZZZ"
        } else {
            self.name.ends_with('Z')
        }
    }
}

fn run(
    start: &str,
    directions: &[Direction],
    nodes: &HashMap<String, Node>,
    full_end: bool,
) -> usize {
    let mut steps = 0;

    let mut idx = 0;
    let mut node = nodes.get(start).unwrap();
    loop {
        steps += 1;

        let direction = directions[idx];

        node = match direction {
            Direction::Left => nodes.get(node.left.as_str()).unwrap(),
            Direction::Right => nodes.get(node.right.as_str()).unwrap(),
        };

        if node.is_end(full_end) {
            break;
        }

        idx = (idx + 1) % directions.len();
    }

    steps
}

fn part1(directions: &[Direction], nodes: &HashMap<String, Node>) {
    let steps = run("AAA", directions, nodes, true);

    assert!(steps == 22411);
    println!("Steps: {}", steps);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part2(directions: &[Direction], nodes: &HashMap<String, Node>) {
    let start_nodes = nodes.values().filter(|n| n.is_start()).collect::<Vec<_>>();

    let steps = start_nodes
        .iter()
        .map(|n| run(n.name.as_str(), directions, nodes, false))
        .collect::<Vec<_>>();

    //println!("steps: {:?}", steps);

    let steps = steps.iter().fold(1, |acc, v| lcm(acc, *v));

    assert!(steps == 11188774513823);
    println!("Steps: {}", steps);
}

fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    let directions_line = lines.next().unwrap();

    let mut directions = vec![];
    for idx in 0..directions_line.len() {
        directions.push(Direction::from_str(&directions_line[idx..idx + 1]).unwrap());
    }

    let nodes = lines
        .filter_map(|v| {
            if v.is_empty() {
                return None;
            }

            let node = Node::from(v);
            Some((node.name.clone(), node))
        })
        .collect::<HashMap<_, _>>();

    part1(&directions, &nodes);
    part2(&directions, &nodes);
}
