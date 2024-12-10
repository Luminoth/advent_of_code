use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
}

fn visit_node(
    heightmap: &[Vec<u32>],
    node: Position,
    prev_height: u32,
    visited: &mut HashSet<Position>,
) -> usize {
    if visited.contains(&node) {
        //println!("not revisiting {:?}", node);
        return 0;
    }

    if heightmap[node.y][node.x] != prev_height + 1 {
        return 0;
    }

    //println!("visiting node {:?}", node);

    visited.insert(node);

    let height = heightmap[node.y][node.x];
    if height == 9 {
        //println!("found peak at {:?}", node);
        return 1;
    }

    let mut score = 0;

    if node.x > 0 {
        score += visit_node(heightmap, node.left(), height, visited);
    }

    if node.x < heightmap[0].len() - 1 {
        score += visit_node(heightmap, node.right(), height, visited);
    }

    if node.y > 0 {
        score += visit_node(heightmap, node.up(), height, visited);
    }

    if node.y < heightmap.len() - 1 {
        score += visit_node(heightmap, node.down(), height, visited);
    }

    score
}

fn start_trailhead(heightmap: &[Vec<u32>], trailhead: Position) -> usize {
    //println!("starting trailhead {:?}", trailhead);

    let mut visited = HashSet::new();
    visited.insert(trailhead);

    let mut score = 0;

    if trailhead.x > 0 {
        score += visit_node(heightmap, trailhead.left(), 0, &mut visited);
    }

    if trailhead.x < heightmap[0].len() - 1 {
        score += visit_node(heightmap, trailhead.right(), 0, &mut visited);
    }

    if trailhead.y > 0 {
        score += visit_node(heightmap, trailhead.up(), 0, &mut visited);
    }

    if trailhead.y < heightmap.len() - 1 {
        score += visit_node(heightmap, trailhead.down(), 0, &mut visited);
    }

    score
}

fn part1(heightmap: &[Vec<u32>]) {
    //println!("heightmap: {:?}", heightmap);

    // TODO: probably a better way to do this
    // if we even need to, we can probably just check the height in the later loop
    let mut trailheads = vec![];
    for (y, line) in heightmap.iter().enumerate() {
        for (x, h) in line.iter().enumerate() {
            if *h == 0 {
                trailheads.push(Position { x, y });
            }
        }
    }

    //println!("found {} trailheads", trailheads.len());

    let mut total = 0;
    for trailhead in trailheads {
        let score = start_trailhead(heightmap, trailhead);
        //println!("trailhead at {:?} has score {}", trailhead, score);
        total += score;
        //println!();
    }

    assert!(total == 461);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");
    let heightmap = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect::<Vec<_>>();

    part1(&heightmap);
}
