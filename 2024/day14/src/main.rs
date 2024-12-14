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

#[derive(Debug, Default)]
struct Robot {
    position: Vector,
    velocity: Vector,
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
            let mut robot = Robot {
                position: robot.position + robot.velocity * 100,
                velocity: robot.velocity,
            };

            // wrap around
            // TODO: there's probably a more correct way to handle negatives
            robot.position.x %= width;
            if robot.position.x < 0 {
                robot.position.x = width + robot.position.x;
            }
            robot.position.y %= height;
            if robot.position.y < 0 {
                robot.position.y = height + robot.position.y;
            }

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
}
