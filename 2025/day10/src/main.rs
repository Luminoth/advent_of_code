#![allow(dead_code)]

#[derive(Debug)]
struct MachineDesc {
    // machine starts when indicator lights match this
    indicator_lights: Vec<bool>,

    // each button is an entry in the outter vec
    // each button is indexes into the lights it toggles
    button_wirings: Vec<Vec<usize>>,

    // ignored for part 1
    joltage_reqs: Vec<usize>,
}

fn part1(machines: impl AsRef<[MachineDesc]>) {
    let machines = machines.as_ref();

    let mut _total = 0;

    for _machine in machines {
        // TODO: calculate min presses and add to total
    }

    //assert!(total == ???);
    println!("Total: {}", _total);
}

impl From<&str> for MachineDesc {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        let indicator_lights = parts
            .next()
            .unwrap()
            .trim_matches(['[', ']'])
            .chars()
            .map(|ch| ch == '#')
            .collect();

        let mut button_wirings = vec![];
        let mut joltage_reqs = vec![];
        for part in parts {
            if part.starts_with("(") {
                let button_wiring = part
                    .trim_matches(['(', ')'])
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect();
                button_wirings.push(button_wiring);
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
            button_wirings,
            joltage_reqs,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let machines = input.lines().map(MachineDesc::from).collect::<Vec<_>>();

    part1(&machines);
}
