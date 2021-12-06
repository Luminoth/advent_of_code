// I originally tried to brute force simulate this one
// but at 256 days that becomes impossible, even with rayon added in to help
// taking inspiration again from https://github.com/zertosh/
// the better approach is to just keep a count of the number of fish at each age
// and use Vec's rotate_left to cycle them

fn simulate(initialages: impl AsRef<[usize]>, days: usize) -> usize {
    // count the number of fish at each initial age
    let mut ages: [usize; 9] = [0; 9];
    for age in initialages.as_ref() {
        ages[*age] += 1;
    }

    // run the simulation
    for _ in 0..days {
        // cycle the set of ages
        // this adds n new fish at age 8
        ages.rotate_left(1);

        // add back in all of the fish
        // that started over at age 6
        ages[6] += ages[8];
    }

    let count: usize = ages.iter().sum();
    println!("After {} days there are {} lanternfish", days, count);

    count
}

fn main() {
    let input = include_str!("../input.txt");

    let initialages: Vec<usize> = input
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

    assert!(simulate(initialages.clone(), 80) == 394994);
    assert!(simulate(initialages, 256) == 1765974267455);
}
