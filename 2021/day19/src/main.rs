use glam::IVec3;

#[inline]
fn rotate_right(mut v: IVec3) -> IVec3 {
    let z = v.z;
    v.z = v.y;
    v.y = v.x;
    v.x = z;

    v
}

#[inline]
fn swap_xy(mut v: IVec3) -> IVec3 {
    let x = v.x;
    v.x = v.y;
    v.y = x;

    v
}

#[inline]
fn swap_yz(mut v: IVec3) -> IVec3 {
    let z = v.z;
    v.z = v.y;
    v.y = z;

    v
}

#[inline]
fn swap_xz(mut v: IVec3) -> IVec3 {
    let z = v.z;
    v.z = v.x;
    v.x = z;

    v
}

fn test_beacons(beacon: IVec3) -> Vec<IVec3> {
    // x, y, z => -x, -y, -z
    let mut beacons = vec![beacon];
    beacons.push(beacon * IVec3::new(-1, 1, 1));
    beacons.push(beacon * IVec3::new(-1, -1, 1));
    beacons.push(beacon * IVec3::new(-1, -1, -1));
    beacons.push(beacon * IVec3::new(1, -1, 1));
    beacons.push(beacon * IVec3::new(1, -1, -1));
    beacons.push(beacon * IVec3::new(1, 1, -1));

    // z, x, y => -z, -x, -y
    let b1 = rotate_right(beacon);
    beacons.push(b1);
    beacons.push(b1 * IVec3::new(-1, 1, 1));
    beacons.push(b1 * IVec3::new(-1, -1, 1));
    beacons.push(b1 * IVec3::new(-1, -1, -1));
    beacons.push(b1 * IVec3::new(1, -1, 1));
    beacons.push(b1 * IVec3::new(1, -1, -1));
    beacons.push(b1 * IVec3::new(1, 1, -1));

    // y, z, x => -y, -z, -x
    let b2 = rotate_right(b1);
    beacons.push(b2);
    beacons.push(b2 * IVec3::new(-1, 1, 1));
    beacons.push(b2 * IVec3::new(-1, -1, 1));
    beacons.push(b2 * IVec3::new(-1, -1, -1));
    beacons.push(b2 * IVec3::new(1, -1, 1));
    beacons.push(b2 * IVec3::new(1, -1, -1));
    beacons.push(b2 * IVec3::new(1, 1, -1));

    // y, x, z => -y, -x, -z
    let b4 = swap_xy(beacon);
    beacons.push(b4);
    beacons.push(b4 * IVec3::new(-1, 1, 1));
    beacons.push(b4 * IVec3::new(-1, -1, 1));
    beacons.push(b4 * IVec3::new(-1, -1, -1));
    beacons.push(b4 * IVec3::new(1, -1, 1));
    beacons.push(b4 * IVec3::new(1, -1, -1));
    beacons.push(b4 * IVec3::new(1, 1, -1));

    // x, z, y => -x, -z, -y
    let b5 = swap_yz(beacon);
    beacons.push(b5);
    beacons.push(b5 * IVec3::new(-1, 1, 1));
    beacons.push(b5 * IVec3::new(-1, -1, 1));
    beacons.push(b5 * IVec3::new(-1, -1, -1));
    beacons.push(b5 * IVec3::new(1, -1, 1));
    beacons.push(b5 * IVec3::new(1, -1, -1));
    beacons.push(b5 * IVec3::new(1, 1, -1));

    // z, y, x => -z, -y, -x
    let b6 = swap_xz(beacon);
    beacons.push(b6);
    beacons.push(b6 * IVec3::new(-1, 1, 1));
    beacons.push(b6 * IVec3::new(-1, -1, 1));
    beacons.push(b6 * IVec3::new(-1, -1, -1));
    beacons.push(b6 * IVec3::new(1, -1, 1));
    beacons.push(b6 * IVec3::new(1, -1, -1));
    beacons.push(b6 * IVec3::new(1, 1, -1));

    beacons
}

#[derive(Debug, Clone)]
struct Scanner {
    position: IVec3,

    beacons: Vec<IVec3>,
}

impl Scanner {
    fn beacon_count(&self) -> usize {
        self.beacons.len()
    }

    fn extents(&self) -> (IVec3, IVec3) {
        let (min, max) = self.beacons.iter().fold(
            (
                IVec3::new(i32::MAX, i32::MAX, i32::MAX),
                IVec3::new(i32::MIN, i32::MIN, i32::MIN),
            ),
            |acc, &x| (acc.0.min(x), acc.1.max(x)),
        );

        (min, max)
    }

    fn beacon(&self, idx: usize) -> IVec3 {
        self.beacons[idx] + self.position
    }

    fn check_overlaps(&self, mut scanner: Scanner, position: IVec3) -> usize {
        scanner.position = position;

        //println!("compare {} to {}", self.position, scanner.position);

        let mut overlapping = 0;
        for idx in 0..scanner.beacon_count() {
            let beacon = scanner.beacon(idx);
            let test = test_beacons(beacon);

            if self.beacons.iter().any(|&x| {
                test.iter().any(|&y| {
                    //println!("comparing {} and {}", x, beacon);
                    if x == y {
                        //println!("overlapping {} and {}", x, y);
                        return true;
                    }

                    //println!("not overlapping {} and {}", x, y);

                    false
                })
            }) {
                overlapping += 1;
            }
        }

        overlapping
    }

    // find the first position where the required number of beacons overlap
    fn overlaps(&self, scanner: &Scanner, required: usize) -> Option<IVec3> {
        let (mut min, mut max) = self.extents();
        min *= 2;
        max *= 2;
        println!("min: {}, max: {}", min, max);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let position = IVec3::new(x, y, z);
                    if position == IVec3::ZERO {
                        println!("zero");
                    }

                    let overlapping = self.check_overlaps(scanner.clone(), position);

                    if overlapping >= required {
                        println!("found it!");
                        return Some(IVec3::default());
                    }
                }
            }
        }

        None
    }
}

impl<T: AsRef<str>> From<T> for Scanner {
    fn from(input: T) -> Self {
        let beacons = input
            .as_ref()
            .lines()
            .skip(1)
            .map(|x| {
                let pos: Vec<i32> = x.trim().split(',').map(|v| v.parse().unwrap()).collect();
                IVec3::new(pos[0], pos[1], pos[2])
            })
            .collect();

        Self {
            position: IVec3::default(),

            beacons,
        }
    }
}

fn part1(scanners: impl AsRef<[Scanner]>) {
    let scanners = scanners.as_ref();

    for (idx, scanner) in scanners.iter().enumerate().skip(1) {
        let overlapped = scanners[0].overlaps(scanner, 6);
        println!("scanner 0 overlapped scanner {} at {:?}", idx, overlapped);
    }
}

fn main() {
    let input = include_str!("../sample.txt").trim();

    let scanners: Vec<Scanner> = input
        .split("\n\n")
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.into())
        })
        .collect();

    part1(&scanners);
}
