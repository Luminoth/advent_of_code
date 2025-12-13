use std::collections::{BTreeSet, HashMap};

fn check_path<'a>(
    nodes: &HashMap<&'a str, BTreeSet<&'a str>>,
    node: &'a str,
    visited: &mut Vec<&'a str>, // was a HashSet in part 1 but part 2 may need to know the order?
    required: &Vec<&str>,
) -> usize {
    if node == "out" {
        for req in required {
            if !visited.contains(req) {
                return 0;
            }
        }
        return 1;
    }

    if visited.contains(&node) {
        println!("cycle");
        return 0;
    }
    visited.push(node);

    let mut paths = 0;

    let connections = nodes.get(node).unwrap();
    for connection in connections {
        let p = check_path(nodes, connection, visited, required);
        //println!("{node} found: {p}");
        paths += p;
    }

    visited.pop();
    paths
}

fn part1(nodes: &HashMap<&str, BTreeSet<&str>>) {
    let paths = check_path(nodes, "you", &mut vec![], &vec![]);

    assert!(paths == 708);
    println!("Paths: {}", paths);
}

fn part2(nodes: &HashMap<&str, BTreeSet<&str>>) {
    let paths = check_path(nodes, "svr", &mut vec![], &vec!["dac", "fft"]);

    //assert!(paths == ??);
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
    part2(&nodes);
}
