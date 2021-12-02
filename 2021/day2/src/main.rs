#[derive(Debug, Default, Clone)]
struct Command {
    pub direction: String,
    pub amount: isize,
}

fn part1(values: impl AsRef<[Command]>) {
    let mut horizontal = 0;
    let mut vertical = 0;

    for value in values.as_ref() {
        match value.direction.as_str() {
            "forward" => horizontal += value.amount,
            "down" => vertical += value.amount,
            "up" => vertical -= value.amount,
            _ => panic!("invalid direction: {}", value.direction),
        }
    }

    assert!(horizontal == 2091);
    assert!(vertical == 721);
    println!(
        "Horizontal: {}, Vertical: {}, Result: {}",
        horizontal,
        vertical,
        horizontal * vertical
    );
}

fn part2(values: impl AsRef<[Command]>) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for value in values.as_ref() {
        match value.direction.as_str() {
            "forward" => {
                horizontal += value.amount;
                depth += aim * value.amount;
            }
            "down" => aim += value.amount,
            "up" => aim -= value.amount,
            _ => panic!("invalid direction: {}", value.direction),
        }
    }

    assert!(horizontal == 2091);
    assert!(depth == 899375);
    println!(
        "Horizontal: {}, Depth: {}, Result: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<Command> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let v: Vec<&str> = x.split(' ').collect();
            assert!(v.len() == 2);

            Some(Command {
                direction: v[0].trim().to_string(),
                amount: v[1].parse().unwrap(),
            })
        })
        .collect();

    part1(&values);
    part2(&values);
}
