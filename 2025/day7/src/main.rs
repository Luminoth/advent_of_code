use std::collections::BTreeSet;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Vector2 {
    pub x: usize,
    pub y: usize,
}

impl Vector2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn part1(manifold: impl AsRef<[Vec<char>]>, start: Vector2) {
    let manifold = manifold.as_ref();

    let mut beams = BTreeSet::new();
    beams.insert(start);

    let mut processing = Vec::new();

    let mut splits = 0;
    loop {
        while let Some(beam) = beams.pop_first() {
            processing.push(beam);
        }

        while let Some(mut beam) = processing.pop() {
            beam.y += 1;
            if beam.y >= manifold.len() {
                continue;
            }

            if manifold[beam.y][beam.x] == '^' {
                if beam.x > 0 {
                    let left = Vector2::new(beam.x - 1, beam.y);
                    beams.insert(left);
                }

                if beam.x < manifold[beam.y].len() - 1 {
                    let right = Vector2::new(beam.x + 1, beam.y);
                    beams.insert(right);
                }

                splits += 1;
            } else {
                beams.insert(beam);
            }
        }

        if beams.is_empty() {
            break;
        }
    }

    assert!(splits == 1546);
    println!("Splits: {}", splits);
}

fn main() {
    let input = include_str!("../input.txt");

    let manifold = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = Vector2::default();
    for x in 0..manifold[0].len() {
        if manifold[0][x] == 'S' {
            start.x = x;
            break;
        }
    }

    part1(&manifold, start);
}
