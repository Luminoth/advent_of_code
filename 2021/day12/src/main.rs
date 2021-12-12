use std::collections::{HashMap, HashSet};

fn is_lowercase(v: impl AsRef<str>) -> bool {
    v.as_ref().chars().all(|ch| ch.is_lowercase())
}

fn part1_visit(
    cave: &'static str,
    graph: &HashMap<&str, Vec<&'static str>>,
    visited: &HashSet<&'static str>,
) -> usize {
    // can't revisit small caves (or start / end)
    if visited.contains(cave) && is_lowercase(cave) {
        assert!(cave != "end");

        return 0;
    }

    // if we hit the end, we have a complete path
    if cave == "end" {
        return 1;
    }

    // clone visited so that we don't interfere with other pathfinding
    let mut visited = visited.clone();
    visited.insert(cave);

    let mut value = 0;

    let current = graph.get(cave).unwrap();
    for cave in current {
        value += part1_visit(cave, graph, &visited);
    }

    value
}

fn part1(graph: HashMap<&str, Vec<&'static str>>) {
    let visited = HashSet::new();
    let total = part1_visit("start", &graph, &visited);

    assert!(total == 5104);
    println!("Total paths: {}", total);
}

fn part2_visit(
    cave: &'static str,
    graph: &HashMap<&str, Vec<&'static str>>,
    visited: &HashSet<&'static str>,
    mut special_small: Option<&'static str>,
) -> usize {
    // only allow revisiting a single small cave once
    // (but never start / end)
    if visited.contains(cave) && is_lowercase(cave) {
        assert!(cave != "end");

        if special_small.is_some() || cave == "start" {
            return 0;
        }

        special_small = Some(cave);
    }

    // if we hit the end, we have a complete path
    if cave == "end" {
        return 1;
    }

    // clone visited so that we don't interfere with other pathfinding
    let mut visited = visited.clone();
    visited.insert(cave);

    let mut value = 0;

    let current = graph.get(cave).unwrap();
    for cave in current {
        value += part2_visit(cave, graph, &visited, special_small);
    }

    value
}

fn part2(graph: HashMap<&str, Vec<&'static str>>) {
    let visited = HashSet::new();
    let total = part2_visit("start", &graph, &visited, None);

    assert!(total == 149220);
    println!("Total paths: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let paths: Vec<(&str, &str)> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let (from, to) = x.split_once('-').unwrap();

            Some((from, to))
        })
        .collect();

    let mut graph = HashMap::new();
    for path in paths {
        graph.entry(path.0).or_insert_with(Vec::new).push(path.1);
        graph.entry(path.1).or_insert_with(Vec::new).push(path.0);
    }

    part1(graph.clone());
    part2(graph);
}
