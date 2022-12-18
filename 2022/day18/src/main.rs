#[derive(Debug)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl From<(usize, usize, usize)> for Cube {
    fn from(v: (usize, usize, usize)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl Cube {
    fn unconnected_sides(&self, cubes: impl AsRef<[Cube]>) -> usize {
        let mut sides = [false; 6];

        // TODO: we could shortcut this a lot to reduce iterations
        for cube in cubes.as_ref() {
            // left
            if self.x == cube.x + 1 && self.y == cube.y && self.z == cube.z {
                sides[0] = true;
            }

            // right
            if self.x + 1 == cube.x && self.y == cube.y && self.z == cube.z {
                sides[1] = true;
            }

            // bottom
            if self.x == cube.x && self.y == cube.y + 1 && self.z == cube.z {
                sides[2] = true;
            }

            // top
            if self.x == cube.x && self.y + 1 == cube.y && self.z == cube.z {
                sides[3] = true;
            }

            // behind
            if self.x == cube.x && self.y == cube.y && self.z == cube.z + 1 {
                sides[4] = true;
            }

            // front
            if self.x == cube.x && self.y == cube.y && self.z + 1 == cube.z {
                sides[5] = true;
            }
        }

        sides.iter().filter(|&x| !x).count()
    }
}

fn part1(cubes: impl AsRef<[Cube]>) {
    let mut total = 0;
    for cube in cubes.as_ref() {
        total += cube.unconnected_sides(&cubes);
    }

    assert!(total == 4460);
    println!("Total unconnected sides: {}", total);
}

fn part2(_cubes: impl AsRef<[Cube]>) {
    // TODO:
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let mut c = x.split(',');
            let x = c.next().unwrap().parse().unwrap();
            let y = c.next().unwrap().parse().unwrap();
            let z = c.next().unwrap().parse().unwrap();

            Some((x, y, z).into())
        })
        .collect::<Vec<_>>();

    part1(&values);
    part2(values);
}
