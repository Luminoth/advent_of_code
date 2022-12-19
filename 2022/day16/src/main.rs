use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use regex::Regex;

const TOTAL_MINUTES: usize = 30;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    _open: RefCell<bool>,

    // tunnels are paths with a distance of 1
    tunnels: HashSet<String>,

    // paths (name, distance) to nodes that aren't directly connected
    paths: RefCell<HashMap<String, usize>>,

    // used during shortest path calculation
    visited: RefCell<bool>,
    distance: RefCell<usize>,
}

impl From<(String, usize, Vec<String>)> for Valve {
    fn from(v: (String, usize, Vec<String>)) -> Self {
        Self {
            name: v.0,
            flow_rate: v.1,
            _open: RefCell::new(false),
            tunnels: HashSet::from_iter(v.2),
            paths: RefCell::new(HashMap::new()),
            visited: RefCell::new(false),
            distance: RefCell::new(usize::MAX),
        }
    }
}

impl Valve {
    // find the shortest path from this node to another node
    fn shortest_path(&self, to: &String, valves: &HashMap<String, Valve>) {
        if self.name == *to || self.tunnels.contains(to) {
            return;
        }

        //println!("finding shortest path from {} to {}", self.name, to);

        assert!(!self.paths.borrow().contains_key(to));

        let to = valves.get(to).unwrap();

        let mut unvisited = vec![];
        for valve in valves.values() {
            *valve.visited.borrow_mut() = false;
            *valve.distance.borrow_mut() = usize::MAX;

            unvisited.push(valve);
        }

        *self.distance.borrow_mut() = 0;

        let mut current = self;
        loop {
            for tunnel in &current.tunnels {
                let n = valves.get(tunnel).unwrap();
                let d = *current.distance.borrow() + 1;
                if d < *n.distance.borrow() {
                    *n.distance.borrow_mut() = d;
                }
            }

            *current.visited.borrow_mut() = true;
            let idx = unvisited
                .iter()
                .position(|x| x.name == current.name)
                .unwrap();
            unvisited.swap_remove(idx);

            let next = unvisited.iter().min_by(|x, y| x.distance.cmp(&y.distance));
            match next {
                Some(next) => {
                    assert!(*next.distance.borrow() != usize::MAX);
                    current = next;
                }
                None => break,
            }
        }

        //println!("  {} to {}: {}", self.name, to.name, to.distance.borrow());

        self.paths
            .borrow_mut()
            .insert(to.name.clone(), *to.distance.borrow());
    }

    // find the shortest path from this node to each other node
    fn shortest_paths(&self, valves: &HashMap<String, Valve>) {
        for valve in valves.values() {
            self.shortest_path(&valve.name, valves);
        }
        assert!(self.tunnels.len() + self.paths.borrow().len() == valves.len() - 1);
    }

    // returns max (visited, minutes, pressure, total)
    // (absolute brute force solution)
    fn highest_pressure_path(
        &self,
        valves: &HashMap<String, Valve>,
        mut minutes: usize,
        mut pressure: usize,
        mut total: usize,
        visited: &mut Vec<String>,
        _depth: usize,
    ) -> (bool, usize, usize, usize) {
        let _pad = _depth * 2;

        // is this valve already visited?
        if visited.contains(&self.name) {
            //println!("{:_pad$}- Already visited {}", ' ', self.name);
            return (false, minutes, pressure, total);
        }

        // have we run out of time (including time to open this valve)?
        let max_minutes = TOTAL_MINUTES - usize::from(self.flow_rate > 0);
        if minutes >= max_minutes {
            //println!("{:_pad$}- Ran out of time to visit {}", ' ', self.name);
            return (false, minutes, pressure, total);
        }

        /*println!(
            "{:_pad$}-Visiting {}: {} minutes, {} pressure, {} total flow",
            ' ', self.name, minutes, pressure, total
        );*/

        // open the valve
        if self.flow_rate > 0 {
            minutes += 1;
            total += pressure;
            pressure += self.flow_rate;
        }

        visited.push(self.name.clone());

        let mut max = (minutes, pressure, total);

        for (path, distance) in self
            .tunnels
            .iter()
            .map(|t| (t, &1))
            .chain(self.paths.borrow().iter())
        {
            let valve = valves.get(path).unwrap();
            let (v, m, p, t) = valve.highest_pressure_path(
                valves,
                minutes + distance,
                pressure,
                total + (distance * pressure),
                visited,
                _depth + 1,
            );
            if !v {
                continue;
            }

            /*println!(
                "{:_pad$}-Visited {} ({}) from {} and got {} minutes, {} pressure, {} total flow",
                ' ', valve.name, path, self.name, m, p, t
            );*/

            // TODO: print what we're comparing

            /*let current = max.2 + ((TOTAL_MINUTES - max.0) * pressure);
            let new = t + ((TOTAL_MINUTES - m) * p);
            if p > max.1 && new >= current {
                max = (m, p, t);
            }*/

            if t > max.2 {
                max = (m, p, t)
            }
        }

        visited.pop();

        /*println!(
            "{:_pad$}-Finished visiting {}: {} minutes, {} pressure, {} total flow",
            ' ', self.name, max.0, max.1, max.2
        );*/

        (true, max.0, max.1, max.2)
    }
}

fn part1(valves: &HashMap<String, Valve>) {
    let mut visited = Vec::with_capacity(valves.len());
    let (_, minutes, pressure, mut total) =
        valves
            .get("AA")
            .unwrap()
            .highest_pressure_path(valves, 0, 0, 0, &mut visited, 0);
    println!(
        "Total: {} ({} pressure in {} minutes)",
        total, pressure, minutes
    );
    total += (TOTAL_MINUTES - minutes) * pressure;

    println!("Total: {}", total);
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

    // this should make the tunnel graph fully connected
    for v in values.values() {
        v.shortest_paths(&values);
    }

    part1(&values);
}
