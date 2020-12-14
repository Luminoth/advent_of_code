fn score(answers: impl AsRef<str>) -> u32 {
    let mut v: u32 = 0;
    for ch in answers.as_ref().chars() {
        if ch.is_whitespace() {
            continue;
        }

        let d = ch as u32 - 'a' as u32;
        v |= 1 << d;
    }
    v
}

fn count_anyone(answers: impl AsRef<str>) -> u32 {
    score(answers).count_ones()
}

fn count_everyone(group_answers: impl AsRef<str>) -> u32 {
    let answers: Vec<&str> = group_answers
        .as_ref()
        .lines()
        .filter(|x| !x.is_empty())
        .collect();

    let mut v: u32 = (0 - 1) as u32;
    for answer in answers {
        let individual = score(answer);
        v &= individual;
    }
    v.count_ones()
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.split("\n\n").filter(|x| !x.is_empty()).collect();

    let count = lines.iter().fold(0, |acc, line| acc + count_anyone(line));
    println!("anyone sum: {}", count);

    let count = lines.iter().fold(0, |acc, line| acc + count_everyone(line));
    println!("everyone sum: {}", count);
}
