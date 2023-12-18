use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cell {
    x: usize,
    y: usize,
    heat_loss: u32,
}

impl From<(usize, usize, char)> for Cell {
    fn from(v: (usize, usize, char)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            heat_loss: v.2.to_digit(10).unwrap(),
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x)))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(grid: &[Vec<Cell>]) {
    let goal = (grid[0].len() - 1, grid.len() - 1);

    let mut dist = grid
        .iter()
        .map(|row| row.iter().map(|_| u32::MAX).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();

    dist[0][0] = 0;
    heap.push(grid[0][0].clone());

    // TODO: this needs some mods to make it not go backwards
    // not go in a straight line for more than 3
    // and actually keep track of the total heat loss

    while let Some(cell) = heap.pop() {
        // TODO: this is all implemented wrong
        if cell.x == goal.0 && cell.y == goal.1 {
            break;
        }

        if cell.heat_loss > dist[cell.y][cell.x] {
            continue;
        }

        for row in grid {
            for next in row {
                let cost = cell.heat_loss + next.heat_loss;
                if cost < dist[next.y][next.x] {
                    heap.push(next.clone());
                    dist[next.y][next.x] = next.heat_loss;
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Cell::from((x, y, c)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&grid);
}
