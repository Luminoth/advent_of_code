use std::cmp::Ordering;
use std::collections::BinaryHeap;

// this is largely taken from the std::collections::binary_heap example

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    risk: usize,
    position: usize,
}

impl State {
    fn new(risk: usize, position: usize) -> Self {
        Self { risk, position }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    node: usize,
    risk: usize,
}

impl Edge {
    fn new(node: usize, risk: usize) -> Self {
        Self { node, risk }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<Edge>>,
}

impl Grid {
    fn safest_path(&self, source: usize, target: usize) -> Option<usize> {
        let mut risk_levels: Vec<_> = (0..self.grid.len()).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        risk_levels[source] = 0;
        heap.push(State::new(0, source));

        while let Some(state) = heap.pop() {
            if state.position == target {
                return Some(state.risk);
            }

            if state.risk > risk_levels[state.position] {
                continue;
            }

            for edge in &self.grid[state.position] {
                let next = State::new(state.risk + edge.risk, edge.node);
                if next.risk < risk_levels[next.position] {
                    heap.push(next);
                    risk_levels[next.position] = next.risk;
                }
            }
        }

        None
    }
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(input: Vec<Vec<usize>>) -> Self {
        let height = input.len();
        let width = input[0].len();

        let mut grid = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let mut edges = Vec::new();

                // up
                if y > 0 {
                    let y = y - 1;
                    edges.push(Edge::new(y * width + x, input[y][x]));
                }

                // left
                if x > 0 {
                    let x = x - 1;
                    edges.push(Edge::new(y * width + x, input[y][x]));
                }

                // down
                if y < height - 1 {
                    let y = y + 1;
                    edges.push(Edge::new(y * width + x, input[y][x]));
                }

                // right
                if x < width - 1 {
                    let x = x + 1;
                    edges.push(Edge::new(y * width + x, input[y][x]));
                }

                grid.push(edges);
            }
        }

        Self { grid }
    }
}

fn expand_nodes(nodes: Vec<Vec<usize>>, times: usize) -> Vec<Vec<usize>> {
    let mut new_rows = Vec::with_capacity(nodes.len());
    for row in &nodes {
        let mut new_row = row.clone();
        for i in 0..times - 1 {
            let updated: Vec<_> = row.iter().map(|v| 1 + ((v + i) % 9)).collect();
            new_row.extend(updated);
        }
        new_rows.push(new_row);
    }

    let mut new_nodes = new_rows.clone();
    for i in 0..times - 1 {
        for row in &new_rows {
            let updated: Vec<_> = row.iter().map(|v| 1 + ((v + i) % 9)).collect();
            new_nodes.push(updated);
        }
    }
    new_nodes
}

fn main() {
    let input = include_str!("../input.txt");

    let nodes: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let row = x
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect();
            Some(row)
        })
        .collect();

    let grid: Grid = nodes.clone().into();
    let total_risk = grid.safest_path(0, grid.grid.len() - 1).unwrap();
    assert!(total_risk == 537);
    println!("The safest path risk level is {}", total_risk);

    let grid: Grid = expand_nodes(nodes, 5).into();
    let total_risk = grid.safest_path(0, grid.grid.len() - 1).unwrap();
    assert!(total_risk == 2881);
    println!("The safest path risk level is {}", total_risk);
}
