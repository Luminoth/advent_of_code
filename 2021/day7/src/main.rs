fn part1(positions: Vec<usize>) {
    let (min, max) = positions
        .iter()
        .fold((0, 0), |acc, &x| (acc.0.min(x), acc.1.max(x)));

    let mut v = vec![0; max - min];
    for position in positions {
        for x in min..max {
            let fuel = (position as isize - x as isize).abs();
            v[x - min] += fuel;
        }
    }

    let cheapest = v.iter().min().unwrap();
    assert!(*cheapest == 323647);
    println!("Cheapest move: {}", cheapest);

    // https://github.com/zertosh/ :
    /*
    positions.sort();

    let median = positions[positions.len() / 2];
    let min_fuel = positions
        .iter()
        .map(|x| if *x < median { median - x } else { x - median })
        .sum::<usize>();
    */
}

fn part2(positions: Vec<usize>) {
    let (min, max) = positions
        .iter()
        .fold((0, 0), |acc, &x| (acc.0.min(x), acc.1.max(x)));

    let mut v = vec![0; max - min];
    for position in positions {
        for x in min..max {
            let fuel = (position as isize - x as isize).abs();
            // (0..=fuel).sum() == fuel * (fuel + 1) / 2
            v[x - min] += fuel * (fuel + 1) / 2;
        }
    }

    let cheapest = v.iter().min().unwrap();
    assert!(*cheapest == 87640209);
    println!("Cheapest move: {}", cheapest);

    // https://github.com/zertosh/ :
    /*
    let avg = input.iter().sum::<usize>() / input.len();
    let min_fuel = input
        .iter()
        .map(|x| if *x < avg { avg - x } else { x - avg })
        .map(|x| {
            (x.pow(2) + x) / 2 // https://en.wikipedia.org/wiki/Summation
        })
        .sum::<usize>();
    */
}

fn main() {
    let input = include_str!("../input.txt");

    let positions: Vec<usize> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let v = x.split(',').map(|x| x.parse().unwrap());
            Some(v)
        })
        .flatten()
        .collect();

    part1(positions.clone());
    part2(positions);
}
