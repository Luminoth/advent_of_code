use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use regex::Regex;

const TOTAL_MINUTES: usize = 30;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    open: RefCell<bool>,
    tunnels: HashSet<String>,

    // paths (name, distance) to nodes that aren't directly connected
    paths: RefCell<HashMap<String, usize>>,

    // used during best path calculation
    visited: RefCell<bool>,
    distance: RefCell<usize>,
    value: RefCell<i64>,
}

impl From<(String, usize, Vec<String>)> for Valve {
    fn from(v: (String, usize, Vec<String>)) -> Self {
        Self {
            name: v.0,
            flow_rate: v.1,
            open: RefCell::new(false),
            tunnels: HashSet::from_iter(v.2),
            paths: RefCell::new(HashMap::new()),
            visited: RefCell::new(false),
            distance: RefCell::new(usize::MAX),
            value: RefCell::new(i64::MIN),
        }
    }
}

impl Valve {
    // find the best path from this node to another node
    fn best_path(&self, to: &String, valves: &HashMap<String, Valve>) {
        if self.name == *to || self.tunnels.contains(to) {
            return;
        }

        //println!("finding best path from {} to {}", self.name, to);

        assert!(!self.paths.borrow().contains_key(to));

        let to = valves.get(to).unwrap();

        let mut unvisited = vec![];
        for valve in valves.values() {
            *valve.visited.borrow_mut() = false;
            *valve.distance.borrow_mut() = usize::MAX;
            *valve.value.borrow_mut() = i64::MIN;

            unvisited.push(valve);
        }

        *self.distance.borrow_mut() = 0;
        *self.value.borrow_mut() = self.flow_rate as i64;

        let mut current = self;
        loop {
            for tunnel in &current.tunnels {
                let n = valves.get(tunnel).unwrap();
                let d = *current.distance.borrow() + 1;
                let v = *current.value.borrow() + n.flow_rate as i64;
                if v >= *n.value.borrow() {
                    *n.distance.borrow_mut() = d;
                    *n.value.borrow_mut() = v;
                }
            }

            *current.visited.borrow_mut() = true;
            let idx = unvisited
                .iter()
                .position(|x| x.name == current.name)
                .unwrap();
            unvisited.swap_remove(idx);

            let next = unvisited.iter().max_by(|x, y| x.value.cmp(&y.value));
            match next {
                Some(next) => {
                    if *next.value.borrow() == i64::MIN {
                        unreachable!();
                    }
                    current = next;
                }
                None => break,
            }
        }

        println!(
            "  {} to {}: {} ({})",
            self.name,
            to.name,
            to.distance.borrow(),
            to.value.borrow()
        );

        self.paths
            .borrow_mut()
            .insert(to.name.clone(), *to.distance.borrow());
    }

    // find the shortest path from this node to each other node
    fn best_paths(&self, valves: &HashMap<String, Valve>) {
        for valve in valves.values() {
            self.best_path(&valve.name, valves);
        }
        assert!(self.tunnels.len() + self.paths.borrow().len() == valves.len() - 1);
    }

    fn value(&self, cost: usize, minutes: usize, pressure: usize, total: usize) -> usize {
        // if we can't get here in time then we have no value
        if minutes + cost >= TOTAL_MINUTES {
            return usize::MIN;
        }

        total + ((cost + 1) * pressure) + self.flow_rate
    }

    fn visit(
        &self,
        valves: &HashMap<String, Valve>,
        minutes: &mut usize,
        pressure: &mut usize,
        total: &mut usize,
        visited: &mut HashSet<String>,
    ) {
        assert!(!visited.contains(&self.name));

        // have we run out of time?
        if *minutes >= TOTAL_MINUTES {
            return;
        }

        println!();
        println!("== Minute {} == ", *minutes + 1);
        println!("Currently at {}.", self.name);
        {
            let mut opened = valves
                .values()
                .filter(|x| *x.open.borrow())
                .map(|x| x.name.clone())
                .collect::<Vec<_>>();
            if !opened.is_empty() {
                opened.sort();
                println!(
                    "Opened valves: {}, releasing {} pressure ({}).",
                    opened.join(", "),
                    pressure,
                    total
                );
            } else {
                println!("No valves are open.");
            }
        }

        if !*self.open.borrow() && self.flow_rate > 0 {
            println!("You open valve {}.", self.name);
            *self.open.borrow_mut() = true;
            *minutes += 1;

            *pressure += self.flow_rate;
            *total += *pressure;

            self.visit(valves, minutes, pressure, total, visited);
            return;
        }

        visited.insert(self.name.clone());

        let paths = self.paths.borrow();

        // find the next highest value, visitable node
        let mut nodes = self
            .tunnels
            .iter()
            .map(|name| (name, &1))
            .chain(paths.iter())
            .filter(|(name, _)| !visited.contains(*name))
            .collect::<Vec<_>>();
        nodes.sort_by(|x, y| {
            let x = valves
                .get(x.0)
                .unwrap()
                .value(*x.1, *minutes, *pressure, *total);
            let y = valves
                .get(y.0)
                .unwrap()
                .value(*y.1, *minutes, *pressure, *total);
            y.cmp(&x)
        });

        println!("  {:?}", nodes);

        let node = nodes.first().unwrap();
        println!("You move to valve {}.", node.0);

        *minutes += node.1;
        *total += node.1 * *pressure;

        let valve = valves.get(nodes.first().unwrap().0).unwrap();
        valve.visit(valves, minutes, pressure, total, visited);
    }
}

fn part1(valves: &HashMap<String, Valve>) {
    let mut minutes = 0;
    let mut pressure = 0;
    let mut total = 0;

    let mut visited = HashSet::new();
    valves.get("AA").unwrap().visit(
        valves,
        &mut minutes,
        &mut pressure,
        &mut total,
        &mut visited,
    );

    println!("Total: {} (finished in {} minutes)", total, minutes);
}

fn main() {
    let re =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let captures = re.captures(x).unwrap();

            let name = captures.get(1).unwrap().as_str().to_owned();
            let rate = captures.get(2).unwrap().as_str().parse().unwrap();
            let connections = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(',')
                .map(|x| x.trim().to_owned())
                .collect();

            Some((name.clone(), (name, rate, connections).into()))
        })
        .collect::<HashMap<_, Valve>>();

    for v in values.values() {
        v.best_paths(&values);
    }

    part1(&values);
}
