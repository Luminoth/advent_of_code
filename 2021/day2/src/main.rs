#[derive(Debug, Default, Clone)]
struct Command {
    pub direction: String,
    pub amount: isize,
}

fn part1(commands: impl AsRef<[Command]>) {
    let mut horizontal = 0;
    let mut vertical = 0;

    for command in commands.as_ref() {
        match command.direction.as_str() {
            "forward" => horizontal += command.amount,
            "down" => vertical += command.amount,
            "up" => vertical -= command.amount,
            _ => unreachable!(),
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

fn part2(commands: impl AsRef<[Command]>) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands.as_ref() {
        match command.direction.as_str() {
            "forward" => {
                horizontal += command.amount;
                depth += aim * command.amount;
            }
            "down" => aim += command.amount,
            "up" => aim -= command.amount,
            _ => unreachable!(),
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

    let commands: Vec<Command> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let (direction, amount) = x.split_once(' ').unwrap();
            Some(Command {
                direction: direction.trim().to_string(),
                amount: amount.parse().unwrap(),
            })
        })
        .collect();

    part1(&commands);
    part2(&commands);
}
