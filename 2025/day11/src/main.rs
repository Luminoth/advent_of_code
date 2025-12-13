use std::collections::{BTreeSet, HashMap, HashSet};

fn check_path<'a>(
    nodes: &HashMap<&'a str, BTreeSet<&'a str>>,
    node: &'a str,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if node == "out" {
        return 1;
    }

    if visited.contains(node) {
        return 0;
    }
    visited.insert(node);

    let mut paths = 0;

    let connections = nodes.get(node).unwrap();
    for connection in connections {
        paths += check_path(nodes, connection, visited);
    }

    visited.remove(node);
    paths
}

fn part1(nodes: &HashMap<&str, BTreeSet<&str>>) {
    let paths = check_path(nodes, "you", &mut HashSet::new());

    assert!(paths == 708);
    println!("Paths: {}", paths);
}

fn main() {
    let input = include_str!("../input.txt");

    let nodes = input
        .lines()
        .map(|line| {
            let (node, connections) = line.split_once(":").unwrap();
            let connections = connections
                .split_ascii_whitespace()
                .collect::<BTreeSet<_>>();
            (node.trim(), connections)
        })
        .collect::<HashMap<_, _>>();

    part1(&nodes);
}
