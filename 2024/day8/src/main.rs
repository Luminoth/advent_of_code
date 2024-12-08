use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn distance(&self, other: &Self) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn extend(&self, x: isize, y: isize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    width: isize,
    height: isize,
    antennas: HashMap<char, Vec<Position>>,
}

// TODO: this is coming up 5 nodes short ...
fn part1(map: &Map) {
    let mut antinodes = HashSet::new();

    #[allow(clippy::for_kv_map)]
    for (_frequency, positions) in &map.antennas {
        for position in positions.iter().combinations(2) {
            let x_dist = (position[1].x - position[0].x).abs();
            let y_dist = (position[1].y - position[0].y).abs();

            let potential_antinoodes = [
                position[0].extend(x_dist, y_dist),
                position[0].extend(-x_dist, -y_dist),
                position[1].extend(x_dist, y_dist),
                position[1].extend(-x_dist, -y_dist),
            ];

            for antinode in potential_antinoodes {
                if antinode.x < 0
                    || antinode.x >= map.width
                    || antinode.y < 0
                    || antinode.y >= map.height
                {
                    continue;
                }

                let da = position[0].distance(&antinode);
                let db = position[1].distance(&antinode);

                if da == 2 * db || 2 * da == db {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    println!("Antinodes: {}", antinodes.len());
}

fn main() {
    let input = include_str!("../input.txt");

    let mut map = Map::default();
    input.lines().enumerate().for_each(|(y, line)| {
        map.height = map.height.max(y as isize + 1);
        for (x, ch) in line.chars().enumerate() {
            map.width = map.width.max(x as isize + 1);
            if ch.is_ascii_alphanumeric() {
                map.antennas.entry(ch).or_default().push(Position {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    });

    part1(&map);
}
