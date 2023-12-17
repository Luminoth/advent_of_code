use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,

    #[default]
    Right,
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
enum Cell {
    #[strum(serialize = ".")]
    Empty { visited: HashSet<Direction> },

    #[strum(serialize = "/")]
    RightMirror { visited: HashSet<Direction> },

    #[strum(serialize = "\\")]
    LeftMirror { visited: HashSet<Direction> },

    #[strum(serialize = "|")]
    VertSplitter { visited: HashSet<Direction> },

    #[strum(serialize = "-")]
    HorizSplitter { visited: HashSet<Direction> },
}

impl Cell {
    fn is_energized(&self) -> bool {
        match self {
            Cell::Empty { visited } => !visited.is_empty(),
            Cell::RightMirror { visited } => !visited.is_empty(),
            Cell::LeftMirror { visited } => !visited.is_empty(),
            Cell::VertSplitter { visited } => !visited.is_empty(),
            Cell::HorizSplitter { visited } => !visited.is_empty(),
        }
    }

    fn encounter(&mut self, mut beam: Beam) -> Vec<Beam> {
        //println!("{:?} encountering {:?}", self, beam);

        match self {
            Cell::Empty { visited } => {
                if !visited.insert(beam.get_direction()) {
                    return vec![];
                }

                vec![beam]
            }
            Cell::RightMirror { visited } => {
                if !visited.insert(beam.get_direction()) {
                    return vec![];
                }

                match beam.get_direction() {
                    Direction::Up => beam.set_direction(Direction::Right),
                    Direction::Down => beam.set_direction(Direction::Left),
                    Direction::Left => beam.set_direction(Direction::Down),
                    Direction::Right => beam.set_direction(Direction::Up),
                }

                vec![beam]
            }
            Cell::LeftMirror { visited } => {
                if !visited.insert(beam.get_direction()) {
                    return vec![];
                }

                match beam.get_direction() {
                    Direction::Up => beam.set_direction(Direction::Left),
                    Direction::Down => beam.set_direction(Direction::Right),
                    Direction::Left => beam.set_direction(Direction::Up),
                    Direction::Right => beam.set_direction(Direction::Down),
                }

                vec![beam]
            }
            Cell::VertSplitter { visited } => {
                if !visited.insert(beam.get_direction()) {
                    return vec![];
                }

                match beam.get_direction() {
                    Direction::Left | Direction::Right => {
                        vec![
                            Beam {
                                x: beam.x,
                                y: beam.y,
                                direction: Direction::Up,
                            },
                            Beam {
                                x: beam.x,
                                y: beam.y,
                                direction: Direction::Down,
                            },
                        ]
                    }
                    Direction::Up | Direction::Down => vec![beam],
                }
            }
            Cell::HorizSplitter { visited } => {
                if !visited.insert(beam.get_direction()) {
                    return vec![];
                }

                match beam.get_direction() {
                    Direction::Up | Direction::Down => {
                        vec![
                            Beam {
                                x: beam.x,
                                y: beam.y,
                                direction: Direction::Left,
                            },
                            Beam {
                                x: beam.x,
                                y: beam.y,
                                direction: Direction::Right,
                            },
                        ]
                    }
                    Direction::Left | Direction::Right => vec![beam],
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Beam {
    fn get_direction(&self) -> Direction {
        self.direction
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn r#move(&mut self, width: i64, height: i64) -> bool {
        let cur_x = self.x as i64;
        let cur_y = self.y as i64;

        let (next_x, next_y) = match self.direction {
            Direction::Up => (cur_x, cur_y - 1),
            Direction::Down => (cur_x, cur_y + 1),
            Direction::Left => (cur_x - 1, cur_y),
            Direction::Right => (cur_x + 1, cur_y),
        };

        /*println!(
            "move from ({},{}) to ({},{}) ({}, {})",
            cur_x, cur_y, next_x, next_y, width, height
        );*/

        if next_x < 0 || next_y < 0 || next_x >= width || next_y >= height {
            //println!("fail at {}, {}", next_x, next_y);
            return false;
        }

        self.x = next_x as usize;
        self.y = next_y as usize;

        true
    }
}

fn count_energized(grid: &Vec<Vec<Cell>>) -> usize {
    let mut total = 0;

    for row in grid {
        for cell in row {
            if cell.is_energized() {
                //print!("#");
                total += 1;
            } else {
                //print!("{}", cell);
            }
        }
        //println!();
    }

    total
}

fn run(
    grid: &mut Vec<Vec<Cell>>,
    start_x: usize,
    start_y: usize,
    start_direction: Direction,
) -> usize {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    let mut beams = VecDeque::new();
    beams.push_back(Beam {
        x: start_x,
        y: start_y,
        direction: start_direction,
    });

    while let Some(beam) = beams.pop_front() {
        let cell = &mut grid[beam.y][beam.x];
        let result = cell.encounter(beam);

        for mut beam in result {
            if beam.r#move(width, height) {
                beams.push_back(beam);
            }
        }
    }

    count_energized(grid)
}

fn part1(mut grid: Vec<Vec<Cell>>) {
    let total = run(&mut grid, 0, 0, Direction::Right);

    assert!(total == 8901);
    println!("Total: {}", total);
}

fn part2(grid: Vec<Vec<Cell>>) {
    let mut max = 0;

    for y in 0..grid.len() {
        let mut test = grid.clone();
        max = run(&mut test, 0, y, Direction::Right).max(max);
        max = run(&mut test, grid[0].len() - 1, y, Direction::Left).max(max);
    }

    for x in 0..grid[0].len() {
        let mut test = grid.clone();
        max = run(&mut test, x, 0, Direction::Down).max(max);
        max = run(&mut test, x, grid.len() - 1, Direction::Up).max(max);
    }

    // TODO: this is giving 9106 but that is too high
    //assert!(max == ???);
    println!("Max: {}", max);
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

    part1(grid.clone());
    part2(grid);
}
