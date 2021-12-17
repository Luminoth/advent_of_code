use std::cmp::Ordering;

use regex::Regex;

fn update_velocity(velocity: (isize, isize)) -> (isize, isize) {
    let x = match 0.cmp(&velocity.0) {
        Ordering::Greater => velocity.0 + 1,
        Ordering::Less => velocity.0 - 1,
        Ordering::Equal => 0,
    };

    (x, velocity.1 - 1)
}

fn update_position(position: (isize, isize), velocity: (isize, isize)) -> (isize, isize) {
    (position.0 + velocity.0, position.1 + velocity.1)
}

fn simulate(
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    initial_velocity: (isize, isize),
) -> Option<isize> {
    let mut position = (0, 0);
    let mut velocity = initial_velocity;

    let mut highest = isize::MIN;
    loop {
        if position.1 > highest {
            highest = position.1;
        }

        // overshot the target
        if position.0 > xmax || position.1 < ymin {
            return None;
        }

        // hit the mark
        if (xmin..=xmax).contains(&position.0) && (ymin..=ymax).contains(&position.1) {
            return Some(highest);
        }

        position = update_position(position, velocity);
        velocity = update_velocity(velocity);
    }
}

fn part1(xmin: isize, xmax: isize, ymin: isize, ymax: isize) {
    let mut maximum = None;

    // this is the most garbage brute force way of doing this ...
    // I'm pretty sure this is a simple math problem to solve in reality
    for vx in 0..xmax {
        for vy in 0..ymin.abs() {
            let highest = simulate(xmin, xmax, ymin, ymax, (vx, vy));
            if let Some(highest) = highest {
                if highest > maximum.unwrap_or(isize::MIN) {
                    maximum = Some(highest);
                }
            }
        }
    }

    let maximum = maximum.unwrap();
    assert!(maximum == 2278);
    println!("Maximum: {}", maximum);
}

fn part2(xmin: isize, xmax: isize, ymin: isize, ymax: isize) {
    let xstart = 0;
    let xend = xmax * 2;
    let ystart = ymin * 2;
    let yend = ymax.abs() * 2;

    // this is the most garbage brute force way of doing this ...
    // I'm pretty sure this is a simple math problem to solve in reality
    let mut hits = 0;
    for vx in xstart..xend {
        for vy in ystart..yend {
            let highest = simulate(xmin, xmax, ymin, ymax, (vx, vy));
            if highest.is_some() {
                hits += 1;
            }
        }
    }

    assert!(hits == 996);
    println!("Velocities that hit the target: {}", hits);
}

fn main() {
    let input = include_str!("../input.txt").trim();

    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let captures = re.captures(input).unwrap();

    let xmin = (&captures[1]).parse().unwrap();
    let xmax = (&captures[2]).parse().unwrap();
    let ymin = (&captures[3]).parse().unwrap();
    let ymax = (&captures[4]).parse().unwrap();

    part1(xmin, xmax, ymin, ymax);
    part2(xmin, xmax, ymin, ymax);
}
