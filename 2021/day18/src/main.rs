use std::collections::VecDeque;

#[derive(Debug)]
enum SnailfishNumberType {
    Number(isize),
    Pair(Box<SnailfishNumber>),
}

impl SnailfishNumberType {
    fn magnitude(&self) -> isize {
        match self {
            Self::Number(number) => *number,
            Self::Pair(number) => number.magnitude(),
        }
    }

    fn split(&self) -> [SnailfishNumberType; 2] {
        match self {
            SnailfishNumberType::Number(number) => [
                SnailfishNumberType::Number(*number / 2),
                SnailfishNumberType::Number((*number as f64 / 2.0).ceil() as isize),
            ],
            _ => panic!("invalid left split!"),
        }
    }
}

impl<T: AsRef<str>> From<T> for SnailfishNumberType {
    fn from(input: T) -> Self {
        let input = input.as_ref();

        let ch = input.chars().next().unwrap();
        if ch == '[' {
            Self::Pair(Box::new(input.into()))
        } else if ch.is_ascii_digit() {
            Self::Number(ch.to_digit(10).unwrap() as isize)
        } else {
            panic!("invalid snailfish number: {}", ch);
        }
    }
}

#[derive(Debug)]
struct SnailfishNumber {
    number: [SnailfishNumberType; 2],
}

impl SnailfishNumber {
    fn magnitude(&self) -> isize {
        3 * self.number[0].magnitude() + 2 * self.number[1].magnitude()
    }

    fn add(self, rhs: SnailfishNumber) -> Self {
        Self {
            number: [
                SnailfishNumberType::Pair(Box::new(self)),
                SnailfishNumberType::Pair(Box::new(rhs)),
            ],
        }
    }

    fn reduce(&mut self, depth: usize) -> bool {
        match &self.number[0] {
            SnailfishNumberType::Number(number) => {
                if *number >= 10 {
                    let number = self.number[0].split();
                    self.number[0] = SnailfishNumberType::Pair(Box::new(Self { number }));
                    return true;
                }
            }
            SnailfishNumberType::Pair(pair) => {
                if depth >= 4 {
                    // TODO: explode... but how?
                }
            }
        }

        match &self.number[1] {
            SnailfishNumberType::Number(number) => {
                if *number >= 10 {
                    let number = self.number[1].split();
                    self.number[1] = SnailfishNumberType::Pair(Box::new(Self { number }));
                    return true;
                }
            }
            SnailfishNumberType::Pair(pair) => {
                if depth >= 4 {
                    // TODO: explode... but how?
                }
            }
        }

        false
    }
}

impl<T: AsRef<str>> From<T> for SnailfishNumber {
    fn from(input: T) -> Self {
        let input = input.as_ref();

        let left = input[1..].into();

        let mut level = 0;
        let mut idx = 0;
        for ch in input.chars() {
            if ch == '[' {
                level += 1;
            } else if ch == ']' {
                level -= 1;
            } else if ch == ',' && level == 1 {
                break;
            }
            idx += 1;
        }
        idx += 1;

        let right = input[idx..].into();

        Self {
            number: [left, right],
        }
    }
}

fn part1(mut numbers: VecDeque<SnailfishNumber>) {
    let mut sum = numbers.pop_front().unwrap();
    for number in numbers {
        sum = sum.add(number);

        loop {
            if !sum.reduce(0) {
                break;
            }
        }
        println!("step: {:?}", sum);
    }

    println!("Final sum: {:?}", sum);
    println!("Sum magnitude: {}", sum.magnitude());
}

fn main() {
    let input = include_str!("../sample.txt");

    let numbers: VecDeque<SnailfishNumber> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let number = x.into();
            Some(number)
        })
        .collect();

    part1(numbers);
}
