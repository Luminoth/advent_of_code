fn count_valid_chars(line: impl AsRef<str>) -> (usize, Vec<char>) {
    let mut stack = Vec::new();
    let valid_count = line
        .as_ref()
        .chars()
        .take_while(|ch| match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(*ch);
                true
            }
            ')' => stack.pop().unwrap() == '(',
            ']' => stack.pop().unwrap() == '[',
            '}' => stack.pop().unwrap() == '{',
            '>' => stack.pop().unwrap() == '<',
            _ => unreachable!(),
        })
        .count();
    (valid_count, stack)
}

fn part1(input: impl AsRef<[&'static str]>) {
    let total: usize = input
        .as_ref()
        .iter()
        .filter_map(|&line| {
            let (valid_count, _) = count_valid_chars(line);
            if valid_count == line.len() {
                None
            } else {
                Some(match line.chars().nth(valid_count).unwrap() {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                })
            }
        })
        .sum();

    assert!(total == 462693);
    println!("Final score: {}", total);
}

fn part2(input: impl AsRef<[&'static str]>) {
    let mut scores: Vec<usize> = input
        .as_ref()
        .iter()
        .filter_map(|&line| {
            let (valid_count, stack) = count_valid_chars(line);
            if valid_count != line.len() {
                None
            } else {
                Some(stack)
            }
        })
        .map(|mut stack| {
            let mut score = 0;
            while !stack.is_empty() {
                score *= 5;
                score += match stack.pop().unwrap() {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }
            score
        })
        .collect();
    assert!(scores.len() % 2 != 0);

    scores.sort_unstable();

    let final_score = scores[scores.len() / 2];
    assert!(final_score == 3094671161);
    println!("Final score: {}", final_score);
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x)
        })
        .collect();

    part1(&lines);
    part2(&lines);
}
