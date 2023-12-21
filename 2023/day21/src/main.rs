use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, strum::EnumString, strum::Display)]
enum Cell {
    #[strum(serialize = "S")]
    Start,

    #[strum(serialize = ".")]
    Garden,

    #[strum(serialize = "#")]
    Rock,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn walk(
    x: usize,
    y: usize,
    steps: usize,
    max: usize,
    grid: &[Vec<Cell>],
    direction: Direction,
    visited: &mut HashSet<(usize, usize)>,
    ends: &mut HashSet<(usize, usize)>,
) {
    //println!("walking {:?} from {}, {}", direction, x, y);

    let width = grid[0].len();
    let height = grid.len();

    match direction {
        Direction::North => {
            if y == 0 {
                return;
            }
            visit(x, y - 1, steps + 1, max, grid, visited, ends)
        }
        Direction::South => {
            if y == height - 1 {
                return;
            }
            visit(x, y + 1, steps + 1, max, grid, visited, ends)
        }
        Direction::East => {
            if x == width - 1 {
                return;
            }
            visit(x + 1, y, steps + 1, max, grid, visited, ends)
        }
        Direction::West => {
            if x == 0 {
                return;
            }
            visit(x - 1, y, steps + 1, max, grid, visited, ends)
        }
    }
}

fn visit(
    x: usize,
    y: usize,
    steps: usize,
    max: usize,
    grid: &[Vec<Cell>],
    visited: &mut HashSet<(usize, usize)>,
    ends: &mut HashSet<(usize, usize)>,
) {
    if grid[y][x] == Cell::Rock {
        //println!("avoiding rock");
        return;
    }

    if steps == max {
        //println!("walked max steps");
        ends.insert((x, y));
        return;
    }

    if visited.contains(&(x, y)) {
        //println!("already visited {}, {}", x, y);
        return;
    }

    println!("visiting {}, {}", x, y);
    visited.insert((x, y));

    walk(x, y, steps, max, grid, Direction::North, visited, ends);
    walk(x, y, steps, max, grid, Direction::South, visited, ends);
    walk(x, y, steps, max, grid, Direction::East, visited, ends);
    walk(x, y, steps, max, grid, Direction::West, visited, ends);

    visited.remove(&(x, y));
}

fn part1(grid: &Vec<Vec<Cell>>) {
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == Cell::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    println!("starting at {}, {}", start.0, start.1);

    let mut visited = HashSet::new();
    let mut ends = HashSet::new();
    for steps in 0..6 {
        visit(start.0, start.1, 0, steps, grid, &mut visited, &mut ends);
    }
    let total = ends.len();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if ends.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }

    //assert!(total == ???);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::from_str(&c.to_string()).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&grid);
}
