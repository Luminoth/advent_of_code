use regex::Regex;

#[derive(Debug)]
struct Move {
    amount: usize,
    start: usize,
    end: usize,
}

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        Self {
            amount: captures.get(1).unwrap().as_str().parse().unwrap(),
            start: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            end: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        }
    }
}

fn part1(mut stacks: Vec<Vec<char>>, moves: impl AsRef<[Move]>) {
    for r#move in moves.as_ref() {
        for _ in 0..r#move.amount {
            let x = stacks[r#move.start].pop().unwrap();
            stacks[r#move.end].push(x);
        }
    }

    let mut top = String::with_capacity(stacks.len());
    for mut stack in stacks {
        top.push(stack.pop().unwrap());
    }

    assert!(top == "GFTNRBZPF");
    println!("Top: {}", top);
}

fn part2(mut stacks: Vec<Vec<char>>, moves: impl AsRef<[Move]>) {
    for r#move in moves.as_ref() {
        let mut s = Vec::with_capacity(r#move.amount);
        for _ in 0..r#move.amount {
            let x = stacks[r#move.start].pop().unwrap();
            s.push(x);
        }
        stacks[r#move.end].extend(s.iter().rev());
    }

    let mut top = String::with_capacity(stacks.len());
    for mut stack in stacks {
        top.push(stack.pop().unwrap());
    }

    assert!(top == "VRQWPDSGP");
    println!("Top: {}", top);
}

fn main() {
    let input = include_str!("../input.txt");

    let (stacks_input, moves) = input.split_once("\n\n").unwrap();

    let stack_count = stacks_input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(vec![]);
    }

    for x in stacks_input.lines().rev().skip(1) {
        for (idx, x) in x.as_bytes().chunks(4).enumerate() {
            let x = std::str::from_utf8(x).unwrap().trim();
            if !x.is_empty() {
                stacks[idx].push(x.chars().nth(1).unwrap());
            }
        }
    }

    let moves = moves
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.into())
        })
        .collect::<Vec<Move>>();

    part1(stacks.clone(), &moves);
    part2(stacks, &moves);
}
