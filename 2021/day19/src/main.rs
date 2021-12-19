#[derive(Debug)]
struct Scanner {
    beacons: Vec<(isize, isize, isize)>,
}

impl<T: AsRef<str>> From<T> for Scanner {
    fn from(input: T) -> Self {
        let beacons = input
            .as_ref()
            .lines()
            .skip(1)
            .map(|x| {
                let pos: Vec<isize> = x.trim().split(',').map(|v| v.parse().unwrap()).collect();
                assert!(pos.len() <= 3);

                (
                    pos.get(0).copied().unwrap_or_default(),
                    pos.get(1).copied().unwrap_or_default(),
                    pos.get(2).copied().unwrap_or_default(),
                )
            })
            .collect();

        Self { beacons }
    }
}

fn part1(scanners: impl AsRef<[Scanner]>) {}

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

    println!("scanners: {:?}", scanners);

    part1(&scanners);
}
