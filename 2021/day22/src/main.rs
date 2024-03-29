use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap()
});

#[derive(Debug)]
enum Toggle {
    On,
    Off,
}

impl<T: AsRef<str>> From<T> for Toggle {
    fn from(input: T) -> Self {
        match input.as_ref() {
            "on" => Self::On,
            "off" => Self::Off,
            _ => panic!("invalid toggle"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    toggle: Toggle,

    x: std::ops::RangeInclusive<isize>,
    y: std::ops::RangeInclusive<isize>,
    z: std::ops::RangeInclusive<isize>,
}

impl Instruction {
    fn on(&self) -> bool {
        matches!(self.toggle, Toggle::On)
    }

    fn is_part1_valid(&self) -> bool {
        (*self.x.start() <= 50 && *self.x.end() >= -50)
            && (*self.y.start() <= 50 && *self.y.end() >= -50)
            && (*self.z.start() <= 50 && *self.z.end() >= -50)
    }
}

impl<T: AsRef<str>> From<T> for Instruction {
    fn from(input: T) -> Self {
        let captures = REGEX.captures(input.as_ref()).unwrap();

        let toggle: Toggle = captures[1].into();

        Self {
            toggle,
            x: captures[2].parse().unwrap()..=captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap()..=captures[5].parse().unwrap(),
            z: captures[6].parse().unwrap()..=captures[7].parse().unwrap(),
        }
    }
}

fn part1(instructions: impl AsRef<[Instruction]>) {
    let mut reactor = HashMap::new();

    for instruction in instructions.as_ref() {
        if !instruction.is_part1_valid() {
            continue;
        }

        for x in instruction.x.clone() {
            if !(-50..=50).contains(&x) {
                continue;
            }

            for y in instruction.y.clone() {
                if !(-50..=50).contains(&y) {
                    continue;
                }

                for z in instruction.z.clone() {
                    if !(-50..=50).contains(&z) {
                        continue;
                    }

                    let coord = (x, y, z);
                    reactor.insert(coord, instruction.on());
                }
            }
        }
    }

    let enabled: usize = reactor.iter().filter(|(_, &v)| v).count();
    assert!(enabled == 623748);
    println!("There are {} valid enabled cubes", enabled);
}

#[allow(dead_code)]
fn part2(instructions: impl AsRef<[Instruction]>) {
    let mut reactor = HashMap::new();

    // TODO: the ranges here are too big for this
    // so we need to find a more optimal way of handling this
    for instruction in instructions.as_ref() {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                for z in instruction.z.clone() {
                    let coord = (x, y, z);
                    reactor.insert(coord, instruction.on());
                }
            }
        }
    }

    let enabled: usize = reactor.iter().filter(|(_, &v)| v).count();
    println!("There are {} enabled cubes", enabled);
}

fn main() {
    let input = include_str!("../input.txt").trim();

    let instructions: Vec<Instruction> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.into())
        })
        .collect();

    part1(instructions);
    //part2(instructions);
}
