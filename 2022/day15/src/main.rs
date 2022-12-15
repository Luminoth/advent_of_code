use regex::Regex;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl From<&str> for Position {
    fn from(v: &str) -> Self {
        let (x, y) = v.split_once(", ").unwrap();

        let (_, x) = x.split_once('=').unwrap();
        let (_, y) = y.split_once('=').unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline]
    fn distance(&self, other: &Position) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs()) as usize
    }
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon_position: Position,
    beacon_distance: usize,
}

impl From<(Position, Position)> for Sensor {
    fn from(v: (Position, Position)) -> Self {
        Self {
            position: v.0,
            beacon_position: v.1,
            beacon_distance: v.0.distance(&v.1),
        }
    }
}

#[derive(Debug)]
struct Map {
    min_x: i64,
    max_x: i64,
    max_y: i64,

    sensors: Vec<Sensor>,
}

impl From<Vec<(Position, Position)>> for Map {
    fn from(v: Vec<(Position, Position)>) -> Self {
        let mut min_x = v.iter().map(|(s, b)| s.x.min(b.x)).min().unwrap();
        let mut max_x = v.iter().map(|(s, b)| s.x.max(b.x)).max().unwrap();
        let mut max_y = v.iter().map(|(s, b)| s.y.max(b.y)).max().unwrap();

        let mut sensors = Vec::with_capacity(v.len());
        for objects in v {
            let sensor: Sensor = objects.into();

            min_x -= sensor.beacon_distance as i64 - 1;
            max_x += sensor.beacon_distance as i64 + 1;
            max_y += sensor.beacon_distance as i64 + 1;

            sensors.push(sensor);
        }

        Self {
            min_x,
            max_x,
            max_y,

            sensors,
        }
    }
}

impl Map {
    fn can_have_beacon_at(&self, position: Position, ignore_known_beacons: bool) -> bool {
        for sensor in &self.sensors {
            // is there a known beacon here?
            if !ignore_known_beacons && sensor.beacon_position == position {
                return true;
            }

            // is this position close enough to the sensor to eliminate?
            if sensor.position.distance(&position) <= sensor.beacon_distance {
                return false;
            }
        }

        true
    }
}

fn part1(map: &Map) {
    // TODO: I must still be missing something here,
    // this is slower than I think it probably should be

    let mut total = 0;
    for x in map.min_x..=map.max_x {
        let position = Position::new(x, 2000000);
        if !map.can_have_beacon_at(position, false) {
            total += 1;
        }
    }

    assert!(total == 6078701);
    println!("{} positions with no beacon", total);
}

fn part2(map: &Map) {
    // TODO: this is very obviously wrong, we can't iterate over all of this garbage

    let max_x = map.max_x.min(4000000);
    let max_y = map.max_y.min(4000000);

    println!("max x: {}, max y: {} ({})", max_x, max_y, max_x * max_y);

    /*let mut signal = Position::default();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let position = Position::new(x, y);
            if map.can_have_beacon_at(position, true) {
                signal = position;
                break;
            }
        }
    }

    let frequency = signal.x * 4000000 + signal.y;
    //assert!(frequency == ???);
    println!("Signal at ({}, {}): {}", signal.x, signal.y, frequency);*/
}

fn main() {
    let re = Regex::new(r"Sensor at (.+): closest beacon is at (.+)").unwrap();

    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let captures = re.captures(x).unwrap();

            let sensor = captures.get(1).unwrap().as_str().into();
            let beacon = captures.get(2).unwrap().as_str().into();

            Some((sensor, beacon))
        })
        .collect::<Vec<_>>();

    let map: Map = values.into();
    part1(&map);
    part2(&map);
}
