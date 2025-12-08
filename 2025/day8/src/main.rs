use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector3 {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Vector3 {
    fn from(value: &str) -> Self {
        let mut parts = value.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}

impl Vector3 {
    fn distance(&self, other: Vector3) -> isize {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derivative::Derivative)]
#[derivative(PartialOrd, Ord)]
struct Pair {
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    a: Vector3,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    b: Vector3,

    distance: isize,
}

impl Pair {
    fn new(a: Vector3, b: Vector3) -> Self {
        Self {
            a,
            b,
            distance: a.distance(b),
        }
    }
}

fn part1(boxes: Vec<Vector3>) {
    let mut pairs = Vec::with_capacity(boxes.len() * (boxes.len() - 1));
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let pair = Pair::new(boxes[i], boxes[j]);
            pairs.push(pair);
        }
    }
    pairs.sort();

    let mut connections = 0;
    let mut _circuits: HashMap<Vector3, HashSet<Vector3>> = HashMap::new();
    for _pair in pairs {
        // TODO: how to store the circuits is the question here
        /*let aentry = circuits.entry(pair.a).or_default();
        aentry.insert(pair.b);

        let bentry = circuits.entry(pair.b).or_default();
        bentry.insert(pair.a);*/

        connections += 1;
        if connections >= 10 {
            break;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let boxes = input.lines().map(Vector3::from).collect::<Vec<_>>();

    part1(boxes);
}
