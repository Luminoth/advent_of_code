use regex::Regex;
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, strum::EnumString)]
enum Direction {
    #[strum(serialize = "U")]
    Up,

    #[strum(serialize = "D")]
    Down,

    #[strum(serialize = "L")]
    Left,

    #[strum(serialize = "R")]
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    meters: usize,

    #[allow(dead_code)]
    color: String,
}

impl From<&str> for Instruction {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"(?<direction>.+) (?<meters>\d+) \((?<color>#.*)\)").unwrap();
        let caps = re.captures(v).unwrap();

        let direction = Direction::from_str(&caps["direction"]).unwrap();
        let meters = caps["meters"].parse::<usize>().unwrap();
        let color = caps["color"].to_owned();

        Self {
            direction,
            meters,
            color,
        }
    }
}

fn edges_to_grid(edges: Vec<(i32, i32)>) -> Vec<Vec<char>> {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for edge in &edges {
        if edge.0 < min_x {
            min_x = edge.0;
        } else if edge.0 > max_x {
            max_x = edge.0;
        }

        if edge.1 < min_y {
            min_y = edge.1;
        } else if edge.1 > max_y {
            max_y = edge.1;
        }
    }

    //println!("{} {} {} {}", min_x, max_x, min_y, max_y);

    // add an outer border to make flood fill easier to implement
    min_y -= 1;
    max_y += 1;
    min_x -= 1;
    max_x += 1;

    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for edge in edges {
        grid[(edge.1 - min_y) as usize][(edge.0 - min_x) as usize] = '#';
    }

    grid
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '.' {
                return (x, y);
            }
        }
    }

    unreachable!();
}

fn fill(grid: &mut [Vec<char>], v: char) {
    let height = grid.len();
    let width = grid[0].len();

    let mut queue = VecDeque::new();
    queue.push_back(find_start(grid));

    while let Some(pos) = queue.pop_front() {
        if grid[pos.1][pos.0] != '.' {
            continue;
        }

        grid[pos.1][pos.0] = v;

        if pos.1 > 0 {
            // N
            queue.push_back((pos.0, pos.1 - 1));

            // NW
            if pos.0 > 0 {
                queue.push_back((pos.0 - 1, pos.1 - 1));
            }

            // NE
            if pos.0 < width - 1 {
                queue.push_back((pos.0 + 1, pos.1 - 1));
            }
        }

        if pos.1 < height - 1 {
            // S
            queue.push_back((pos.0, pos.1 + 1));

            // SW
            if pos.0 > 0 {
                queue.push_back((pos.0 - 1, pos.1 + 1));
            }

            // SE
            if pos.0 < width - 1 {
                queue.push_back((pos.0 + 1, pos.1 + 1));
            }
        }

        if pos.0 > 0 {
            // E
            queue.push_back((pos.0 - 1, pos.1));
        }

        if pos.0 < width - 1 {
            // W
            queue.push_back((pos.0 + 1, pos.1));
        }
    }
}

fn part1(plan: &[Instruction]) {
    let mut x = 0;
    let mut y = 0;
    let mut edges = vec![(x, y)];
    for instruction in plan {
        for _ in 0..instruction.meters {
            match instruction.direction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }
            edges.push((x, y));
        }
    }
    edges.truncate(edges.len() - 1);

    let mut grid = edges_to_grid(edges);

    // as described, there should be an outer area and a singular inner area
    fill(&mut grid, 'o');
    fill(&mut grid, '#');

    let total = grid.iter().flatten().filter(|c| **c == '#').count();

    /*for row in &grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }*/

    assert!(total == 40131);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let plan = input
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    part1(&plan);
}
