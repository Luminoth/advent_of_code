use std::collections::{BTreeSet, HashMap, HashSet};

fn find_cycle<'a>(
    start: &str,
    current: &str,
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    visited: &mut BTreeSet<&'a str>,
    cycles: &mut HashSet<BTreeSet<&'a str>>,
    depth: usize,
) {
    // base case, check for connection to the start
    if depth == 1 {
        if connections.get(current).unwrap().contains(start) {
            cycles.insert(visited.clone());
        }
        return;
    }

    for connection in connections.get(current).unwrap() {
        if visited.contains(connection) {
            continue;
        }

        visited.insert(*connection);
        find_cycle(start, connection, connections, visited, cycles, depth - 1);
        visited.remove(*connection);
    }
}

fn part2(connections: &HashMap<&str, HashSet<&str>>) {
    let mut visited = BTreeSet::new();
    let mut cycles = HashSet::new();
    for computer in connections.keys() {
        visited.insert(*computer);
        find_cycle(
            computer,
            computer,
            connections,
            &mut visited,
            &mut cycles,
            connections.len(),
        );
        visited.remove(*computer);
    }

    /*println!(
        "Longest: {}",
        longest.iter().copied().collect::<Vec<_>>().join(",")
    );*/
}

fn part1(connections: &HashMap<&str, HashSet<&str>>) {
    let mut visited = BTreeSet::new();
    let mut cycles = HashSet::new();
    for computer in connections.keys() {
        visited.insert(*computer);
        find_cycle(
            computer,
            computer,
            connections,
            &mut visited,
            &mut cycles,
            3,
        );
        visited.remove(*computer);
    }

    // count cycles that include a valid computer
    let mut total = 0;
    for cycle in cycles {
        for computer in cycle {
            if computer.starts_with('t') {
                total += 1;
                break;
            }
        }
    }

    assert!(total == 1378);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let computers = line.split_once('-').unwrap();
        connections
            .entry(computers.0)
            .or_default()
            .insert(computers.1);
        connections
            .entry(computers.1)
            .or_default()
            .insert(computers.0);
    });

    part1(&connections);
    part2(&connections);
}
