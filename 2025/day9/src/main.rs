#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let parts = value.split_once(",").unwrap();
        Self {
            x: parts.0.parse().unwrap(),
            y: parts.1.parse().unwrap(),
        }
    }
}

fn part1(tiles: impl AsRef<[Point]>) {
    let tiles = tiles.as_ref();

    // naive algorithm but I'm actually not sure you can do better?
    let mut max_area = 0;
    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            let a = tiles[i];
            let b = tiles[j];

            // + 1 because the points are 0-based
            let w = (b.x - a.x).abs() + 1;
            let h = (b.y - a.y).abs() + 1;
            let area = w * h;

            if area > max_area {
                max_area = area;
            }
        }
    }

    assert!(max_area == 4776487744);
    println!("Area: {}", max_area);
}

fn main() {
    let input = include_str!("../input.txt");

    let tiles = input.lines().map(Point::from).collect::<Vec<_>>();

    part1(tiles);
}
