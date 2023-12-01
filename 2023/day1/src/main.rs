fn part1(values: impl AsRef<[&'static str]>) {
    let mut sum = 0;
    for value in values.as_ref() {
        let first = value.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last = value.chars().rev().find(|x| x.is_ascii_digit()).unwrap();
        let value = format!("{}{}", first, last).parse::<usize>().unwrap();
        sum += value;
    }

    assert!(sum == 54916);
    println!("Sum: {}", sum);
}

fn part2(values: impl AsRef<[&'static str]>) {
    let mut sum = 0;
    for value in values.as_ref() {
        // forward
        let mut scratch = String::new();
        let mut first = None;
        for ch in value.chars() {
            if ch.is_ascii_digit() {
                first = Some(ch.to_string());
                break;
            } else {
                scratch.push(ch);
                if scratch.contains("one") {
                    first = Some("1".to_string());
                    break;
                } else if scratch.contains("two") {
                    first = Some("2".to_string());
                    break;
                } else if scratch.contains("three") {
                    first = Some("3".to_string());
                    break;
                } else if scratch.contains("four") {
                    first = Some("4".to_string());
                    break;
                } else if scratch.contains("five") {
                    first = Some("5".to_string());
                    break;
                } else if scratch.contains("six") {
                    first = Some("6".to_string());
                    break;
                } else if scratch.contains("seven") {
                    first = Some("7".to_string());
                    break;
                } else if scratch.contains("eight") {
                    first = Some("8".to_string());
                    break;
                } else if scratch.contains("nine") {
                    first = Some("9".to_string());
                    break;
                }
            }
        }

        // reverse
        let mut scratch = String::new();
        let mut last = None;
        for ch in value.chars().rev() {
            if ch.is_ascii_digit() {
                last = Some(ch.to_string());
                break;
            } else {
                scratch.push(ch);
                if scratch.contains("eno") {
                    last = Some("1".to_string());
                    break;
                } else if scratch.contains("owt") {
                    last = Some("2".to_string());
                    break;
                } else if scratch.contains("eerht") {
                    last = Some("3".to_string());
                    break;
                } else if scratch.contains("ruof") {
                    last = Some("4".to_string());
                    break;
                } else if scratch.contains("evif") {
                    last = Some("5".to_string());
                    break;
                } else if scratch.contains("xis") {
                    last = Some("6".to_string());
                    break;
                } else if scratch.contains("neves") {
                    last = Some("7".to_string());
                    break;
                } else if scratch.contains("thgie") {
                    last = Some("8".to_string());
                    break;
                } else if scratch.contains("enin") {
                    last = Some("9".to_string());
                    break;
                }
            }
        }

        let value = format!("{}{}", first.unwrap(), last.unwrap())
            .parse::<usize>()
            .unwrap();
        sum += value;
    }

    assert!(sum == 54728);
    println!("Sum: {}", sum);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input.lines().collect::<Vec<_>>();

    part1(&values);
    part2(values);
}
