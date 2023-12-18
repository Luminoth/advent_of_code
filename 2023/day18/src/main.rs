use regex::Regex;
use std::str::FromStr;

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

    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for edge in &edges {
        grid[(edge.1 - min_y) as usize][(edge.0 - min_x) as usize] = '#';
    }

    //println!("{} {} {} {}", min_x, max_x, min_y, max_y);

    // TODO: determining inside vs outside is something that I need to work on figuring out how to do
    // because I'm failing on every question that requires it

    let mut total = 0;
    for row in grid {
        let mut started = false;
        let mut inside = false;
        for cell in row {
            if cell == '#' {
                if !started {
                    started = true;
                } else if inside {
                    started = false;
                    inside = false;
                }
                total += 1;
            } else if cell == '.' && started {
                inside = true;
                total += 1;
            }

            print!("{}", cell);
        }
        println!();
    }

    //assert!(total == ???);
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
