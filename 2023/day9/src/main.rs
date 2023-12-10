#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Forward,
    Reverse,
}

fn process_sequence(sequence: &[i64], direction: Direction) -> i64 {
    //println!("sequence: {:?}", sequence);
    let diffs = if direction == Direction::Forward {
        sequence
            .windows(2)
            .map(|window| {
                //println!("chunk: {:?}", window);
                window[1] - window[0]
            })
            .collect::<Vec<_>>()
    } else {
        sequence
            .windows(2)
            .rev()
            .map(|window| {
                //println!("chunk: {:?}", window);
                window[1] - window[0]
            })
            .rev()
            .collect::<Vec<_>>()
    };

    if diffs.iter().all(|v| *v == 0) {
        return 0;
    }

    //println!("diffs: {:?}", diffs);

    let diff = process_sequence(&diffs, direction);
    //println!("diff: {}", diff);
    if direction == Direction::Forward {
        diffs.iter().last().unwrap() + diff
    } else {
        diffs.first().unwrap() - diff
    }
}

fn part1(sequences: &[Vec<i64>]) {
    let mut total = 0;
    for sequence in sequences {
        let next_value =
            sequence.iter().last().unwrap() + process_sequence(sequence, Direction::Forward);
        //println!("next value: {}", next_value);
        total += next_value;
    }

    assert!(total == 2105961943);
    println!("Total: {}", total);
}

fn part2(sequences: &[Vec<i64>]) {
    let mut total = 0;
    for sequence in sequences {
        let next_value =
            sequence.iter().next().unwrap() - process_sequence(sequence, Direction::Reverse);
        //println!("next value: {}", next_value);
        total += next_value;
    }

    assert!(total == 1019);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");
    let sequences = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&sequences);
    part2(&sequences);
}
