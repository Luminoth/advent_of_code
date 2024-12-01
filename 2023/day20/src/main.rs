use std::cell::RefCell;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
enum Module {
    FlipFlop {
        name: String,
        value: RefCell<bool>,
        destinations: Vec<String>,
    },
    Conjunction {
        name: String,
        values: RefCell<HashMap<String, bool>>,
        destinations: Vec<String>,
    },
    Broadcaster {
        name: String,
        destinations: Vec<String>,
    },
}

impl From<&str> for Module {
    fn from(v: &str) -> Self {
        let re =
            Regex::new(r"(?<type>%|&|broadcaster)(?<name>.+)? -> (?<destinations>.+)").unwrap();
        let caps = re.captures(v).unwrap();

        let r#type = &caps["type"];
        let name = caps.name("name").map(|v| v.as_str().to_owned());
        let destinations = caps["destinations"]
            .split(", ")
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

        match r#type {
            "%" => Self::FlipFlop {
                name: name.unwrap(),
                value: RefCell::new(false),
                destinations,
            },
            "&" => Self::Conjunction {
                name: name.unwrap(),
                values: RefCell::new(HashMap::new()),
                destinations,
            },
            "broadcaster" => Self::Broadcaster {
                name: "broadcaster".to_string(),
                destinations,
            },
            _ => unreachable!(),
        }
    }
}

impl Module {
    fn get_name(&self) -> &String {
        match self {
            Self::FlipFlop { name, .. } => name,
            Self::Conjunction { name, .. } => name,
            Self::Broadcaster { name, .. } => name,
        }
    }

    fn get_destinations(&self) -> &Vec<String> {
        match self {
            Self::FlipFlop { destinations, .. } => destinations,
            Self::Conjunction { destinations, .. } => destinations,
            Self::Broadcaster { destinations, .. } => destinations,
        }
    }

    fn broadcast(
        input: &String,
        pulse: bool,
        destinations: &Vec<String>,
        modules: &HashMap<String, Module>,
    ) -> (usize, usize) {
        let mut counts = (0, 0);

        // TODO: this should be breadth-first, not depth-first
        for destination in destinations {
            println!(
                "{} -{}-> {}",
                input,
                if pulse { "high" } else { "low" },
                destination
            );
            let (highs, lows) = if let Some(module) = modules.get(destination) {
                module.handle(input, pulse, modules)
            } else {
                (0, 0)
            };

            counts.0 += highs;
            counts.1 += lows;
        }

        counts
    }

    fn handle(
        &self,
        input: &String,
        pulse: bool,
        modules: &HashMap<String, Module>,
    ) -> (usize, usize) {
        match self {
            Self::FlipFlop {
                name,
                value,
                destinations,
                ..
            } => {
                if pulse {
                    (0, 0)
                } else {
                    let prev = *value.borrow();
                    *value.borrow_mut() = !prev;

                    let value = *value.borrow();
                    let (highs, lows) = Self::broadcast(name, value, destinations, modules);

                    if value {
                        (destinations.len() + highs, lows)
                    } else {
                        (highs, destinations.len() + lows)
                    }
                }
            }
            Self::Conjunction {
                name,
                values,
                destinations,
                ..
            } => {
                *values.borrow_mut().get_mut(input).unwrap() = pulse;

                if values.borrow().values().all(|v| *v) {
                    let (highs, lows) = Self::broadcast(name, false, destinations, modules);
                    (highs, destinations.len() + lows)
                } else {
                    let (highs, lows) = Self::broadcast(name, true, destinations, modules);
                    (destinations.len() + highs, lows)
                }
            }
            Self::Broadcaster {
                name, destinations, ..
            } => {
                let (highs, lows) = Self::broadcast(name, pulse, destinations, modules);
                if pulse {
                    (destinations.len() + highs, lows)
                } else {
                    (highs, destinations.len() + lows)
                }
            }
        }
    }
}

fn part1(modules: HashMap<String, Module>) {
    let broadcaster = modules.get("broadcaster").unwrap();

    let mut totals = (0, 0);
    //for _ in 0..1000 {
    println!("button -low-> broadcaster");
    let (highs, lows) = broadcaster.handle(broadcaster.get_name(), false, &modules);
    totals.0 += highs;
    totals.1 += lows + 1;
    //}

    let total = totals.0 * (totals.1);
    //assert!(total == 489392);
    println!("Total: {} (highs: {}, lows: {})", total, totals.0, totals.1);
}

fn main() {
    let input = include_str!("../input.txt");

    let modules = input
        .lines()
        .map(|line| {
            let module = Module::from(line);
            (module.get_name().to_owned(), module)
        })
        .collect::<HashMap<_, _>>();

    for module in modules.values() {
        if let Module::Conjunction { name, values, .. } = module {
            for input in modules
                .values()
                .filter(|m| m.get_destinations().contains(name))
            {
                values.borrow_mut().insert(input.get_name().clone(), false);
            }
        }
    }

    //println!("{:#?}", modules);

    part1(modules);
}
