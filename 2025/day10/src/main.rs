#![allow(dead_code)]

use itertools::Itertools;

// lots of help on this from https://www.reddit.com/r/adventofcode/comments/1pity70/2025_day_10_solutions/

#[derive(Debug)]
struct MachineDesc {
    // machine starts when indicator lights match this
    indicator_lights: Vec<bool>,
    indicator_lights_value: usize, // the lights as an int

    // each button is an entry in the outter vec
    // each button is indexes into the lights it toggles
    button_wirings: Vec<Vec<usize>>,
    button_values: Vec<usize>, // the buttons as an int

    // ignored for part 1
    joltage_reqs: Vec<usize>,
}

fn part1(machines: impl AsRef<[MachineDesc]>) {
    let machines = machines.as_ref();

    /*
    the buttons can be pressed in any order and the result will be the same
    pressing a button twice is the same as never pressing it
    therefore only need to consider pressing each button at most once
    if we have n buttons then that's 2 ^ n combos (we either press each button or we don't)
    this is basically just the power set
    */

    let mut total = 0;

    for machine in machines {
        let count = machine
            .button_values
            .iter()
            .powerset()
            .filter_map(|pressed| {
                let on = pressed.iter().fold(0, |acc, &n| acc ^ n);
                if on == machine.indicator_lights_value {
                    Some(pressed.len())
                } else {
                    None
                }
            })
            .min()
            .unwrap();

        total += count;
    }

    assert!(total == 500);
    println!("Total: {}", total);
}

fn part2(machines: impl AsRef<[MachineDesc]>) {
    let _machines = machines.as_ref();

    // TODO: apparently this can be solved with z3
}

impl From<&str> for MachineDesc {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        let mut indicator_lights_value = 0;
        let indicator_lights = parts
            .next()
            .unwrap()
            .trim_matches(['[', ']'])
            .chars()
            .rev() // reverse the light order (make them little endian)
            .map(|ch| {
                let v = ch == '#';
                indicator_lights_value <<= 1;
                indicator_lights_value |= v as usize;
                v
            })
            .collect();

        let mut button_wirings = vec![];
        let mut button_values = vec![];
        let mut joltage_reqs = vec![];
        for part in parts {
            if part.starts_with("(") {
                let mut button_value = 0;
                let button_wiring = part
                    .trim_matches(['(', ')'])
                    .split(",")
                    .map(|s| {
                        let v = s.parse().unwrap();
                        button_value |= 1 << v as u32;
                        v
                    })
                    .collect();
                button_wirings.push(button_wiring);
                button_values.push(button_value);
            } else {
                let joltage_req = part
                    .trim_matches(['{', '}'])
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                joltage_reqs.extend(joltage_req);
            }
        }

        Self {
            indicator_lights,
            indicator_lights_value,
            button_wirings,
            button_values,
            joltage_reqs,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let machines = input.lines().map(MachineDesc::from).collect::<Vec<_>>();

    part1(&machines);
    part2(&machines);
}
