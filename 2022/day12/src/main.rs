use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Node {
    position: Position,
    index: usize,
    height: usize,

    visited: bool,
    distance: usize,
}

impl Node {
    fn reset(&mut self) {
        self.visited = false;
        self.distance = usize::MAX;
    }

    fn can_visit(&self, dst: &NodeHandle) -> bool {
        // is this node already visited?
        if dst.borrow().visited {
            return false;
        }

        let srch = self.height as i32;
        let dsth = dst.borrow().height as i32;
        let d = dsth - srch;
        d <= 1
    }

    fn visit(&self, dst: &NodeHandle) {
        if !self.can_visit(dst) {
            return;
        }

        let d = self.distance + 1;
        if d < dst.borrow().distance {
            dst.borrow_mut().distance = d;
        }
    }
}

impl From<(Position, usize, usize)> for Node {
    fn from(v: (Position, usize, usize)) -> Self {
        Self {
            position: v.0,
            index: v.1,
            height: v.2,
            visited: false,
            distance: usize::MAX,
        }
    }
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Grid {
    nodes: Vec<Vec<NodeHandle>>,
}

impl Grid {
    fn reset(&self) {
        for row in &self.nodes {
            for col in row {
                col.borrow_mut().reset();
            }
        }
    }

    fn visit(&self, unvisited: &mut Vec<NodeHandle>, start: Position) {
        let mut p = start;
        loop {
            let current = self.nodes[p.y][p.x].clone();
            /*println!(
                "visiting {} ({}, {}) (h: {}, d: {})",
                current.borrow().index,
                current.borrow().position.x,
                current.borrow().position.y,
                current.borrow().height,
                current.borrow().distance
            );*/

            // up
            if p.y > 0 {
                let p = Position::new(p.x, p.y - 1);
                let n = self.nodes[p.y][p.x].clone();
                current.borrow().visit(&n);
            }

            // down
            if p.y < self.nodes.len() - 1 {
                let p = Position::new(p.x, p.y + 1);
                let n = self.nodes[p.y][p.x].clone();
                current.borrow().visit(&n);
            }

            // left
            if p.x > 0 {
                let p = Position::new(p.x - 1, p.y);
                let n = self.nodes[p.y][p.x].clone();
                current.borrow().visit(&n);
            }

            // right
            if p.x < self.nodes[p.y].len() - 1 {
                let p = Position::new(p.x + 1, p.y);
                let n = self.nodes[p.y][p.x].clone();
                current.borrow().visit(&n);
            }

            // current node is now visited
            current.borrow_mut().visited = true;
            let idx = unvisited
                .iter()
                .position(|x| x.borrow().index == current.borrow().index)
                .unwrap();
            unvisited.swap_remove(idx);

            let next = unvisited
                .iter()
                .min_by(|x, y| x.borrow().distance.cmp(&y.borrow().distance));
            match next {
                Some(next) => {
                    if next.borrow().distance == usize::MAX {
                        return;
                    }
                    p = next.borrow().position;
                }
                None => return,
            }
        }
    }

    fn shortest_path(&self, start: Position) {
        let mut unvisited = vec![];
        for row in &self.nodes {
            for node in row {
                unvisited.push(node.clone());
            }
        }

        self.nodes[start.y][start.x].borrow_mut().distance = 0;

        self.visit(&mut unvisited, start);
    }
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(grid: Vec<Vec<usize>>) -> Self {
        let mut nodes = Vec::with_capacity(grid.len());
        for (y, row) in grid.iter().enumerate() {
            let mut v = Vec::with_capacity(row.len());
            for (x, &h) in row.iter().enumerate() {
                let idx = y * row.len() + x;
                v.push(Rc::new(RefCell::new((Position::new(x, y), idx, h).into())));
            }
            nodes.push(v);
        }

        Self { nodes }
    }
}

fn part1(grid: impl Into<Grid>, start: Position, end: Position) {
    let grid = grid.into();
    grid.shortest_path(start);

    let distance = grid.nodes[end.y][end.x].borrow().distance;
    assert!(distance == 352);
    println!("Shortest path: {}", distance);
}

// this is... not optimal lol
fn part2(grid: impl Into<Grid>, end: Position) {
    let grid = grid.into();

    let mut min_start = None;
    let mut min = usize::MAX;

    for y in 0..grid.nodes.len() {
        for x in 0..grid.nodes[y].len() {
            if grid.nodes[y][x].borrow().height == 0 {
                grid.reset();

                let start = Position::new(x, y);
                grid.shortest_path(start);

                let distance = grid.nodes[end.y][end.x].borrow().distance;
                if distance < min {
                    min = distance;
                    min_start = Some(start);
                }
            }
        }
    }

    println!("Shortest path: {} (from {:?})", min, min_start.unwrap());
}

fn main() {
    let input = include_str!("../input.txt");

    let mut start = Position::default();
    let mut end = Position::default();
    let values = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let r = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let c = match c {
                        'S' => {
                            start = Position::new(x, y);
                            'a'
                        }
                        'E' => {
                            end = Position::new(x, y);
                            'z'
                        }
                        _ => c,
                    };
                    c as usize - 'a' as usize
                })
                .collect::<Vec<_>>();

            Some(r)
        })
        .collect::<Vec<_>>();

    part1(values.clone(), start, end);
    part2(values, end);
}
