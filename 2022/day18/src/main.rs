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

fn part2(cubes: impl AsRef<[Cube]>) {
    let minx = cubes.as_ref().iter().map(|x| x.x).min().unwrap();
    let maxx = cubes.as_ref().iter().map(|x| x.x).max().unwrap() + 1;
    let xlen = maxx - minx;

    let miny = cubes.as_ref().iter().map(|x| x.y).min().unwrap();
    let maxy = cubes.as_ref().iter().map(|x| x.y).max().unwrap() + 1;
    let ylen = maxy - miny;

    let minz = cubes.as_ref().iter().map(|x| x.z).min().unwrap();
    let maxz = cubes.as_ref().iter().map(|x| x.z).max().unwrap() + 1;
    let zlen = maxz - minz;

    println!(
        "size: {}x{}x{}: {} ({} sides)",
        xlen,
        ylen,
        zlen,
        xlen * ylen * zlen,
        cubes.as_ref().len() * 6
    );

    let mut total = 0;

    // thinking (NOTE: this is incorrect, we could have adjacent disjoint sections):
    //   in 2d it would be for each y if there is a cube then 1 side is exposed there
    //   in 3d it would be for each (y, z) if there is a cube then 1 side is exposed there?
    //   repeat for y = (x,z), z = (x, y)

    // left
    for y in miny..maxy {
        for z in minz..maxz {
            for cube in cubes.as_ref() {
                if cube.y == y && cube.z == z {
                    total += 2;
                    break;
                }
            }
        }
    }

    // y
    for x in miny..maxy {
        for z in minz..maxz {
            for cube in cubes.as_ref() {
                if cube.x == x && cube.z == z {
                    total += 2;
                    break;
                }
            }
        }
    }

    // z
    for x in miny..maxy {
        for y in minz..maxz {
            for cube in cubes.as_ref() {
                if cube.x == x && cube.y == y {
                    total += 2;
                    break;
                }
            }
        }
    }

    println!("Total unconnected sides: {}", total);

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
