fn calculate_wins(time: usize, distance: usize) -> usize {
    let mut wins = 0;
    for hold in 0..=time {
        let v = hold;
        let d = v * (time - hold);
        if d > distance {
            wins += 1;
        }
    }
    wins
}

fn part1(times: &str, distances: &str) {
    let times = times
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let distances = distances
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut total_wins = vec![];
    for (time, distance) in times.iter().zip(distances.iter()) {
        let wins = calculate_wins(*time, *distance);
        total_wins.push(wins);
    }

    let total = total_wins.iter().product::<usize>();
    assert!(total == 345015);
    println!("Total: {}", total);
}

fn part2(mut time: String, mut distance: String) {
    time.retain(|c| !c.is_ascii_whitespace());
    let time = time.parse::<usize>().unwrap();
    distance.retain(|c| !c.is_ascii_whitespace());
    let distance = distance.parse::<usize>().unwrap();

    let wins = calculate_wins(time, distance);
    assert!(wins == 42588603);
    println!("Total: {}", wins);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();

    let times = lines.next().unwrap().split_once(' ').unwrap();
    let distances = lines.next().unwrap().split_once(' ').unwrap();

    part1(times.1, distances.1);
    part2(times.1.to_owned(), distances.1.to_owned());
}
