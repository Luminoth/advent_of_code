use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    Small(&'static str),
    Large(&'static str),
    End,
}

impl From<&'static str> for Cave {
    fn from(v: &'static str) -> Self {
        if v == "start" {
            Self::Start
        } else if v == "end" {
            Self::End
        } else if v.chars().all(|ch| ch.is_lowercase()) {
            Self::Small(v)
        } else {
            Self::Large(v)
        }
    }
}

impl Cave {
    fn name(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Small(name) => name,
            Self::Large(name) => name,
            Self::End => "end",
        }
    }
}

fn part1_visit(cave: Cave, graph: &HashMap<&str, Vec<Cave>>, mut visited: HashSet<Cave>) -> usize {
    // can't revisit small caves (or start / end)
    if !matches!(cave, Cave::Large(_)) && visited.contains(&cave) {
        assert!(cave != Cave::End);

        return 0;
    }

    // if we hit the end, we have a complete path
    if cave == Cave::End {
        return 1;
    }

    visited.insert(cave);

    let mut value = 0;

    let current = graph.get(cave.name()).unwrap();
    for cave in current {
        value += part1_visit(*cave, graph, visited.clone());
    }

    value
}

fn part1(graph: HashMap<&str, Vec<Cave>>) {
    let total = part1_visit(Cave::Start, &graph, HashSet::new());

    assert!(total == 5104);
    println!("Total paths: {}", total);
}

fn part2_visit(
    cave: Cave,
    graph: &HashMap<&str, Vec<Cave>>,
    mut visited: HashSet<Cave>,
    mut special_small: Option<Cave>,
) -> usize {
    // only allow revisiting a single small cave once
    // (but never start / end)
    if !matches!(cave, Cave::Large(_)) && visited.contains(&cave) {
        assert!(cave != Cave::End);

        if special_small.is_some() || cave == Cave::Start {
            return 0;
        }

        special_small = Some(cave);
    }

    // if we hit the end, we have a complete path
    if cave == Cave::End {
        return 1;
    }

    visited.insert(cave);

    let mut value = 0;

    let current = graph.get(cave.name()).unwrap();
    for cave in current {
        value += part2_visit(*cave, graph, visited.clone(), special_small);
    }

    value
}

fn part2(graph: HashMap<&str, Vec<Cave>>) {
    let total = part2_visit(Cave::Start, &graph, HashSet::new(), None);

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
        graph
            .entry(path.0)
            .or_insert_with(Vec::new)
            .push(path.1.into());
        graph
            .entry(path.1)
            .or_insert_with(Vec::new)
            .push(path.0.into());
    }

    part1(graph.clone());
    part2(graph);
}
