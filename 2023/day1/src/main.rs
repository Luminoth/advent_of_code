fn part1(values: impl AsRef<[&'static str]>) {
    let mut sum = 0;
    for value in values.as_ref() {
        let first = value
            .chars()
            .find(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .unwrap();
        let last = value
            .chars()
            .rev()
            .find(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .unwrap();
        sum += first * 10 + last;
    }

    assert!(sum == 54916);
    println!("Sum: {}", sum);
}

fn to_value(substr: impl AsRef<str>) -> Option<u32> {
    let substr = substr.as_ref();
    if substr.contains("one") {
        return Some(1);
    } else if substr.contains("two") {
        return Some(2);
    } else if substr.contains("three") {
        return Some(3);
    } else if substr.contains("four") {
        return Some(4);
    } else if substr.contains("five") {
        return Some(5);
    } else if substr.contains("six") {
        return Some(6);
    } else if substr.contains("seven") {
        return Some(7);
    } else if substr.contains("eight") {
        return Some(8);
    } else if substr.contains("nine") {
        return Some(9);
    }
    None
}

fn part2(values: impl AsRef<[&'static str]>) {
    let mut sum = 0;
    for value in values.as_ref() {
        let mut first = None;
        for (idx, ch) in value.chars().enumerate() {
            if ch.is_ascii_digit() {
                first = ch.to_digit(10);
                break;
            } else {
                first = to_value(&value[0..=idx]);
                if first.is_some() {
                    break;
                }
            }
        }

        let mut last = None;
        for (idx, ch) in value.chars().rev().enumerate() {
            if ch.is_ascii_digit() {
                last = ch.to_digit(10);
                break;
            } else {
                let idx = value.len() - idx - 1;
                last = to_value(&value[idx..value.len()]);
                if last.is_some() {
                    break;
                }
            }
        }

        sum += first.unwrap() * 10 + last.unwrap();
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
