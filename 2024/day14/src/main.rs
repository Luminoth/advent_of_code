use std::ops::{Add, Mul};

#[derive(Debug, Default, Copy, Clone)]
struct Vector {
    x: isize,
    y: isize,
}

impl Mul<isize> for Vector {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn simulate(&mut self, seconds: isize) {
        self.position = self.position + self.velocity * seconds;
    }

    fn wrap(&mut self, width: isize, height: isize) {
        // TODO: there's probably a more correct way to handle negatives

        self.position.x %= width;
        if self.position.x < 0 {
            self.position.x += width;
        }

        self.position.y %= height;
        if self.position.y < 0 {
            self.position.y += height;
        }
    }
}

fn part1(robots: &[Robot], width: isize, height: isize) {
    assert!(width % 2 == 1);
    assert!(height % 2 == 1);

    let mw = width / 2;
    let mh = height / 2;

    let quadrants = robots
        .iter()
        .map(|robot| {
            // move the robot
            let mut robot = robot.clone();
            robot.simulate(100);
            robot.wrap(width, height);

            robot
        })
        .fold((0, 0, 0, 0), |mut acc, robot| {
            // count the quadrants
            if robot.position.x < mw && robot.position.y < mh {
                acc.0 += 1;
            } else if robot.position.x > mw && robot.position.y < mh {
                acc.1 += 1;
            } else if robot.position.x < mw && robot.position.y > mh {
                acc.2 += 1;
            } else if robot.position.x > mw && robot.position.y > mh {
                acc.3 += 1;
            }
            acc
        });

    let score = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
    assert!(score == 219150360);
    println!("score: {}", score);
}

fn part2(mut robots: Vec<Robot>, width: isize, height: isize) {
    let mut grid = vec![vec![0_usize; width as usize]; height as usize];
    for robot in &robots {
        grid[robot.position.y as usize][robot.position.x as usize] += 1;
    }

    let mut seconds = 0;
    'outer: loop {
        seconds += 1;

        for robot in robots.iter_mut() {
            grid[robot.position.y as usize][robot.position.x as usize] -= 1;
            robot.simulate(1);
            robot.wrap(width, height);
            grid[robot.position.y as usize][robot.position.x as usize] += 1;
        }

        // I cheated and checked reddit for wtf we're looking for here
        // there's an outline so just look for a line of that
        for row in &grid {
            let mut c = 0;
            let mut m = 0;
            for x in 1..row.len() - 1 {
                if row[x - 1] > 0 && row[x] > 0 && row[x + 1] > 0 {
                    c += 1;
                } else {
                    m = m.max(c);
                    c = 0;
                }
            }

            // rough estimate that seems to work
            if m > 10 {
                /*for row in &grid {
                    for col in row {
                        if *col != 0 {
                            print!("*");
                        } else {
                            print!(" ");
                        }
                    }
                    println!();
                }
                println!();*/

                break 'outer;
            }
        }

        // reddit says we cycle back to our
        // initial position in this many seconds
        if seconds >= width * height {
            break;
        }
    }

    assert!(seconds == 8053);
    println!("seconds: {}", seconds);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut width = 0;
    let mut height = 0;

    let robots = input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();

            let position = &parts.0[2..].split_once(',').unwrap();
            let position = Vector {
                x: position.0.parse().unwrap(),
                y: position.1.parse().unwrap(),
            };

            width = width.max(position.x + 1);
            height = height.max(position.y + 1);

            let velocity = &parts.1[2..].split_once(',').unwrap();
            let velocity = Vector {
                x: velocity.0.parse().unwrap(),
                y: velocity.1.parse().unwrap(),
            };

            Robot { position, velocity }
        })
        .collect::<Vec<_>>();

    part1(&robots, width, height);
    part2(robots, width, height);
}
