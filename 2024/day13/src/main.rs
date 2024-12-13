use regex::Regex;

#[derive(Debug)]
struct Puzzle {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

fn part2(puzzles: &[Puzzle]) {
    let mut total = 0;

    for puzzle in puzzles {
        let prize = (
            puzzle.prize.0 + 10000000000000,
            puzzle.prize.1 + 10000000000000,
        );

        let x = (puzzle.b.0 * prize.1 - puzzle.b.1 * prize.0)
            / (puzzle.b.0 * puzzle.a.1 - puzzle.b.1 * puzzle.a.0);
        let y = (puzzle.a.1 * prize.0 - puzzle.a.0 * prize.1)
            / (puzzle.a.1 * puzzle.b.0 - puzzle.a.0 * puzzle.b.1);

        if x * puzzle.a.0 + y * puzzle.b.0 != prize.0 || x * puzzle.a.1 + y * puzzle.b.1 != prize.1
        {
            continue;
        }

        let cost = x * 3 + y * 1;
        total += cost;
    }

    assert!(total == 83197086729371);
    println!("Total cost: {}", total);
}

fn part1(puzzles: &[Puzzle]) {
    let mut total = 0;

    for puzzle in puzzles {
        let x = (puzzle.b.0 * puzzle.prize.1 - puzzle.b.1 * puzzle.prize.0)
            / (puzzle.b.0 * puzzle.a.1 - puzzle.b.1 * puzzle.a.0);
        let y = (puzzle.a.1 * puzzle.prize.0 - puzzle.a.0 * puzzle.prize.1)
            / (puzzle.a.1 * puzzle.b.0 - puzzle.a.0 * puzzle.b.1);

        if (x > 100 || y > 100)
            || (x * puzzle.a.0 + y * puzzle.b.0 != puzzle.prize.0
                || x * puzzle.a.1 + y * puzzle.b.1 != puzzle.prize.1)
        {
            continue;
        }

        let cost = x * 3 + y * 1;
        total += cost;
    }

    assert!(total == 37297);
    println!("Total cost: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let button_a_regex = Regex::new(r"Button A: X\+(?P<x>\d+), Y\+(?P<y>\d+)").unwrap();
    let button_b_regex = Regex::new(r"Button B: X\+(?P<x>\d+), Y\+(?P<y>\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(?P<x>\d+), Y=(?P<y>\d+)").unwrap();

    let puzzles = input
        .split("\n\n")
        .map(|puzzle| {
            let mut lines = puzzle.lines();

            let line = lines.next().unwrap();
            let caps = button_a_regex.captures(line).unwrap();
            let a = (caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            let line = lines.next().unwrap();
            let caps = button_b_regex.captures(line).unwrap();
            let b = (caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            let line = lines.next().unwrap();
            let caps = prize_regex.captures(line).unwrap();
            let prize = (caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            Puzzle { a, b, prize }
        })
        .collect::<Vec<_>>();

    part1(&puzzles);
    part2(&puzzles);
}
