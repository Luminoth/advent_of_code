use std::collections::{HashMap, HashSet};

fn part1(_nodes: &HashMap<&str, HashSet<&str>>) {}

fn main() {
    let input = include_str!("../input.txt");

    let nodes = input
        .lines()
        .map(|line| {
            let (node, connections) = line.split_once(":").unwrap();
            let connections = connections.split_ascii_whitespace().collect::<HashSet<_>>();
            (node.trim(), connections)
        })
        .collect::<HashMap<_, _>>();

    part1(&nodes);
}
