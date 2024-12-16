use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn update_distance(
    map: &[Vec<Tile>],
    distances: &mut HashMap<Position, usize>,
    prev: &mut HashMap<Position, Position>,
    u: Position,
    v: Position,
    turned: bool,
) {
    let mut alt = distances[&u];
    if map[v.y][v.x] == Tile::Wall {
        alt = usize::MAX;
    } else {
        alt += 1;
        if turned {
            alt += 1000;
        }
    }

    if alt < distances[&v] {
        distances.insert(v, alt);
        prev.insert(v, u);
    }
}

fn part1(map: &[Vec<Tile>], start: Position, end: Position) {
    // TODO: unvisited / distances should be a heap or something right?
    // so we always pull the min and don't have to iterate it?
    let mut unvisited = HashSet::new();
    let mut distances = HashMap::new();
    let mut prev = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let p = Position::new(x, y);
            distances.insert(p, usize::MAX);
            unvisited.insert(p);
        }
    }
    distances.insert(start, 0);

    // TODO: how do we update this?
    // track it in prev?
    let direction = Direction::East;

    while !unvisited.is_empty() {
        let mut min_d = usize::MAX;
        let mut u = None;
        for p in &unvisited {
            let d = distances[p];
            if d < min_d {
                min_d = d;
                u = Some(*p);
            }
        }

        if u.is_none() {
            panic!("no possible route!");
        }

        let u = u.unwrap();
        if u == end {
            println!("found the end");
            break;
        }
        unvisited.remove(&u);

        update_distance(
            map,
            &mut distances,
            &mut prev,
            u,
            u.up(),
            direction != Direction::North,
        );
        update_distance(
            map,
            &mut distances,
            &mut prev,
            u,
            u.down(),
            direction != Direction::South,
        );
        update_distance(
            map,
            &mut distances,
            &mut prev,
            u,
            u.left(),
            direction != Direction::West,
        );
        update_distance(
            map,
            &mut distances,
            &mut prev,
            u,
            u.right(),
            direction != Direction::East,
        );
    }

    println!("Total: {}", distances[&end]);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut start = Position::default();
    let mut end = Position::default();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => {
                        start = Position::new(x, y);
                        Tile::Empty
                    }
                    'E' => {
                        end = Position::new(x, y);
                        Tile::Empty
                    }
                    _ => unreachable!("{}", ch),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&map, start, end);
}
